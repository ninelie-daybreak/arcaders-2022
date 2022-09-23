use crate::phi::{Phi, View, ViewAction};
use crate::phi::data::{Rectangle, MaybeAlive};
use crate::phi::gfx::{Sprite, CopySprite, AnimatedSprite, AnimatedSpriteDescr};
use crate::views::shared::Background;
use crate::views::main_menu::MainMenuView;
use sdl2::pixels::Color;

// Constants about asteroid
const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROID_WIDE: usize = 21;
const ASTEROID_HIGH: usize = 7;
const ASTEROID_TOTAL: usize = ASTEROID_WIDE * ASTEROID_HIGH - 4;
const ASTEROID_SIDE: f64 = 96.0;

// Constants about the explosion
const EXPLOSION_PATH: &'static str = "assets/explosion.png";
const EXPLOSIONS_WIDE: usize = 5;
const EXPLOSIONS_HIGH: usize = 4;
const EXPLOSIONS_TOTAL: usize = 17;
const EXPLOSION_SIDE: f64 = 96.0;
const EXPLOSION_FPS: f64 = 16.0;
const EXPLOSION_DURATION: f64 = 1.0 / EXPLOSION_FPS * EXPLOSIONS_TOTAL as f64;

/// Pixels traveled by the player's ship every second, when it is moving
const PLAYER_SPEED:f64 = 180.0;

//? The velocity shared by all bullets, in pixels per second.
const BULLET_SPEED: f64 = 240.0;

// Constants about the ship
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

//? The size of the rectangle which will represent the bullet.
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;

const DEBUG: bool = false;

/// The different states our ship might be in. In the image, they're ordered
/// from left to right, then top to bottom.
#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}

#[derive(Clone)]
struct Asteroid {
    sprite: AnimatedSprite,
    rect: Rectangle,
    vel: f64,
}

impl Asteroid {
    fn factory(phi: &mut Phi) -> AsteroidFactory {
        AsteroidFactory {
            sprite: AnimatedSprite::with_fps(
                AnimatedSprite::load_frames(phi, AnimatedSpriteDescr {
                    image_path: ASTEROID_PATH,
                    total_frames: ASTEROID_TOTAL,
                    frames_high: ASTEROID_HIGH,
                    frames_wide: ASTEROID_WIDE,
                    frame_w: ASTEROID_SIDE,
                    frame_h: ASTEROID_SIDE,
                }), 1.0),
        }
    }

    fn update(mut self, dt: f64) -> Option<Asteroid>{
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);

        if self.rect.x <= -ASTEROID_SIDE {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        if DEBUG {
            // Render the bounding box.
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.rect().to_sdl()).unwrap();
        }

        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }
}

struct AsteroidFactory {
    sprite: AnimatedSprite,
}

impl AsteroidFactory {
    fn random(&self, phi: &mut Phi) -> Asteroid {
        let (w, h) = phi.output_size();

        // FPS in [10.0, 30.0)
        let mut sprite = self.sprite.clone();
        sprite.set_fps(::rand::random::<f64>().abs() * 20.0 + 10.0);

        Asteroid {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally
            rect: Rectangle {
                w: ASTEROID_SIDE,
                h: ASTEROID_SIDE,
                x: w,
                y: ::rand::random::<f64>().abs() * (h - ASTEROID_SIDE),
            },
            vel: ::rand::random::<f64>().abs() * 100.0 + 50.0,
        }
    }
}

struct Explosion {
    sprite: AnimatedSprite,
    rect: Rectangle,

    // Keep how long its been arived, so that we destroy the explosion once
    // its animation is finished.
    alive_since: f64,
}

impl Explosion {
    fn factory(phi: &mut Phi) -> ExplosionFactory {
        ExplosionFactory {
            sprite: AnimatedSprite::with_fps(
                AnimatedSprite::load_frames(phi, AnimatedSpriteDescr {
                    image_path: EXPLOSION_PATH,
                    total_frames: EXPLOSIONS_TOTAL,
                    frames_high: EXPLOSIONS_HIGH,
                    frames_wide: EXPLOSIONS_WIDE,
                    frame_w: EXPLOSION_SIDE,
                    frame_h: EXPLOSION_SIDE,
                }), EXPLOSION_FPS),
        }
    }

    fn update(mut self, dt: f64) -> Option<Explosion> {
        self.alive_since += dt;
        self.sprite.add_time(dt);

        if self.alive_since >= EXPLOSION_DURATION {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
}

struct ExplosionFactory {
    sprite: AnimatedSprite,
}

impl ExplosionFactory {
    fn at_center(&self, center: (f64, f64)) -> Explosion {
        // FPS in [10.0, 30.0)
        let mut sprite = self.sprite.clone();

        Explosion {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally
            rect: Rectangle::with_size(EXPLOSION_SIDE, EXPLOSION_SIDE).center_at(center),

            alive_since: 0.0,
        }
    }
}

struct RectBullet {
    rect: Rectangle,
}

struct SineBullet {
    //? Notice that the bounding box isn't stored directly. This means that
    //? we do not keep useless innformation. It also implies that we must compute
    //? the `sin` function every time we attempt to get the bounding box.
    pos_x: f64,
    origin_y: f64,
    amplitude: f64, 
    angular_vel: f64,
    total_time: f64,
}

/// Bullet which follows a vertical trajectory given by:
///     a * ((t / b)^3 - (t / b)^2)
struct DevergentBullet {
    pos_x : f64,
    origin_y : f64,
    a: f64,
    b: f64,
    total_time: f64,
}

trait Bullet: {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen
    /// then return `None`.
    /// Otherwise, return `Some(update_bullet)`
    /// 
    /// Notice how we use `Box<Self> as the type of `self`. This means: keep
    /// this data behind a pointer, but `move` the pointer. You should note that
    /// we are not copying the value: we are only copying the _address_ at
    /// which the value is stored in memory, which has a negligible cost. We can
    /// do this because Rust will automatically free the memory once the `Box` that
    /// contains it is itself destroyed.
    fn update(self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<dyn Bullet>>;

    /// Render the bullet to the screen.
    /// Here, we take an immutable reference to the bullet, because we do not
    /// need to change its value to draw it, This is the same as before.
    fn render(&self, phi: &mut Phi);

    /// Return the bullet's bounding box.
    fn rect(&self) -> Rectangle;
}

impl Bullet for RectBullet {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen
    /// then return `None`.
    /// Otherwise, return `Some(update_bullet)`
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<dyn Bullet>> {
        let (w, _) = phi.output_size();
        self.rect.x += BULLET_SPEED * dt;

        // If the bullet has left the screen then delete it.
        if self.rect.x > w {
            None
        } else {
            Some(self)
        }
    }

    /// Render the bullet to the screen.
    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect.to_sdl()).unwrap();
    }

    /// Return the bullet's bounding box.
    fn rect(&self) -> Rectangle {
        self.rect
    }
}

impl Bullet for SineBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<dyn Bullet>> {
        //? We store the total time...
        self.total_time += dt;

        //? And move at the same speed as regular bullets.
        self.pos_x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        let (w, _) = phi.output_size();

        if self.rect().x > w{
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl()).unwrap();
    }

    fn rect(&self) -> Rectangle {
        //? Just the general form of the sine function, minus the initial time.
        let dy = self.amplitude * f64::sin(self.angular_vel * self.total_time);
        Rectangle {
            x: self.pos_x,
            y: self.origin_y + dy,
            w: BULLET_W,
            h: BULLET_H,
        }
    }
}

impl Bullet for DevergentBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<dyn Bullet>>{
        self.total_time += dt;
        self.pos_x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        let (w, h) = phi.output_size();
        let rect = self.rect();

        if rect.x > w || rect.x < 0.0 ||
           rect.y > h || rect.y < 0.0 {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl()).unwrap();
    }

    fn rect(&self) -> Rectangle {
        let dy = self.a * 
                            ((self.total_time / self.b).powi(3) - 
                             (self.total_time / self.b).powi(2));
        Rectangle {
            x: self.pos_x,
            y: self.origin_y + dy,
            w: BULLET_W,
            h: BULLET_H,
        }
    }
}

#[derive(Clone,Copy)]
enum CannonType {
    RectBullet,
    SineBullet { amplitude: f64, angular_vel: f64 },
    DevergentBullet { a: f64, b: f64},
}

#[derive(Clone)]
struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
    cannon: CannonType,
}

impl Ship {
    fn spawn_bullets(&self) -> Vec<Box<dyn Bullet>> {
        let cannons_x = self.rect.x + 30.0;
        let cannons1_y = self.rect.y + 6.0;
        let cannons2_y = self.rect.y + SHIP_H - 10.0;

        // One bullet at the tip of every cannon

        match self.cannon {
            CannonType::RectBullet => 
                vec![
                    Box::new(RectBullet {
                        rect: Rectangle {
                            x: cannons_x,
                            y: cannons1_y,
                            w: BULLET_W,
                            h: BULLET_H,
                        }
                    }),
                    Box::new(RectBullet {
                        rect: Rectangle {
                            x: cannons_x,
                            y: cannons2_y,
                            w: BULLET_W,
                            h: BULLET_H,
                        }
                    }),
                ],

            CannonType::SineBullet { amplitude, angular_vel } =>
                vec![
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannons1_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannons2_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                ],
            
            CannonType::DevergentBullet { a, b } => 
                vec![
                    // If a,b > 0, eventually goes upwards
                    Box::new(DevergentBullet {
                        pos_x: cannons_x,
                        origin_y: cannons1_y,
                        a: -a,
                        b: b,
                        total_time: 0.0,
                    }),

                    // If a,b > 0, eventually goes downwards
                    Box::new(DevergentBullet{
                        pos_x: cannons_x,
                        origin_y: cannons2_y,
                        a: a,
                        b: b,
                        total_time: 0.0,
                    })
                ]
        }
    }
}

pub struct GameView {
    player: Ship,
    bullets: Vec<Box<dyn Bullet>>,
    asteroids: Vec<Asteroid>,
    asteroid_factory: AsteroidFactory,
    explosions: Vec<Explosion>,
    explosion_factory: ExplosionFactory,

    bg_back: Background,
    bg_middle: Background,
    bg_front: Background,
}

impl GameView {
    pub fn new(phi: &mut Phi) -> GameView {
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        
        //? When we know in advance how many elements the `Vec` we contain, we 
        //? can allocate the good amount of data up-front.
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        GameView {
            player: Ship { 
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
                /// Let `RectBullet` be the default kind of bullet.
                cannon: CannonType::RectBullet,
            },

            /// We start with no bullets. Because the size of the vector will
            /// change drastically throughout the program, there is not much
            /// point in giving it a capacity.
            bullets: vec![],
            asteroids: vec![],
            asteroid_factory: Asteroid::factory(phi),
            explosions: vec![],
            explosion_factory: Explosion::factory(phi),

            bg_back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starBG.png").unwrap(),
            },

            bg_middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starMG.png").unwrap(),
            },

            bg_front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(&mut phi.renderer, "assets/starFG.png").unwrap(),
            }
        }
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit;
        }

        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(
                crate::views::main_menu::MainMenuView::new(phi)
            ))
        }

        // Change the player's cannonsself.player.cannon = CannonType::RectBullet;
        if phi.events.now.key_1 == Some(true) {
            self.player.cannon = CannonType::RectBullet;
        }

        if phi.events.now.key_2 == Some(true) {
            self.player.cannon = CannonType::SineBullet { 
                amplitude: 10.0,
                angular_vel: 15.0,
            }
        }

        if phi.events.now.key_3 == Some(true) {
            self.player.cannon = CannonType::DevergentBullet {
                a: 100.0,
                b: 1.2,
            }
        }

        // Moving logic
        let diagonal = 
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved = 
            if diagonal { 1.0 / 2.0f64.sqrt()}
            else { 1.0 } * PLAYER_SPEED * elapsed;
        
        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // The movable region spans the entire height of the window and 70% of its
        // width. This way, the player cannot get to the far right of the screen, where
        // we will spawn the asnewteroids, and get immediately eliminated.
        //
        // We restrain the width because most screens are wider than they are high.
        let movable_region = Rectangle { 
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // If the player cannot fit in the screen, then there is a problem and
        // the game should be promptly aborted.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        // Select the appropriate sprite of the ship to show.
        self.player.current = 
            if dx == 0.0 && dy < 0.0       { ShipFrame::UpNorm }
            else if dx > 0.0 && dy < 0.0   { ShipFrame::UpFast }
            else if dx < 0.0 && dy < 0.0   { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
            else if dx > 0.0 && dy == 0.0  { ShipFrame::MidFast }
            else if dx < 0.0 && dy == 0.0  { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
            else if dx > 0.0 && dy > 0.0   { ShipFrame::DownFast }
            else if dx < 0.0 && dy > 0.0   { ShipFrame::DownSlow }
            else { unreachable!() };
        
        
        // Set `self.bullets` to be the empty vector, and put its content inside of 
        // `old_bullets`, which we can move without borrow-checker issues.
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);
        
        // Upon assignment, the old value of `self.bullets`, namely the empty vector,
        // will be freed automatically, because its owner no longer refers to it.
        // We can then update the bullet quite simply.
        self.bullets = 
            old_bullets.into_iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();
        
        // Update the asteroid
        self.asteroids =
            ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| asteroid.update(elapsed))
            .collect();

        // Update the explosion
        self.explosions = 
            ::std::mem::replace(&mut self.explosions, vec![])
            .into_iter()
            .filter_map(|explosion| explosion.update(elapsed))
            .collect();
        
        // Collision detection

        //? We keep track of whether or not the player is alive.
        let mut player_alive = true;

        //? First, go through the bullets and wrap them in a `MaybeAlive`, so that
        //? we can keep track of which got into a collision and which did not
        let mut transition_bullets: Vec<_> = 
            ::std::mem::replace(&mut self.bullets, vec![])
            .into_iter()
            .map(|bullet| MaybeAlive { alive: true, value: bullet })
            .collect();

        self.asteroids = 
            ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| {
                // By default, the asteroid has not been in a collision.
                let mut asteroid_alive = true;

                for bullet in &mut transition_bullets {
                    //? Notice that we refer to the bullet as `bullet.value`
                    //? because it has been wrapped in `MaybeAlive`.
                    if asteroid.rect().overlaps(bullet.value.rect()) {
                        asteroid_alive = false;
                        //? We go through every bullet and `kill` those that collide
                        //? with the asteroid. We do this for every asteroid.
                        bullet.alive = false;
                    }
                }

                //? The player's ship is destroyed if it is hit by an asteroid.
                //? In which case, the asteroid is also destroyed.
                if asteroid.rect().overlaps(self.player.rect) {
                    asteroid_alive = false;
                    player_alive = false;
                }

                //? Then, we use the magic of `filter_map` to keep only the asteroids
                //? that didn't explode.
                if asteroid_alive {
                    Some(asteroid)
                } else {
                    // Spawn an explosive whenever an asteroid was destroyed.
                    self.explosions.push(
                        self.explosion_factory.at_center(
                            asteroid.rect().center()));
                    None
                }
            })
            .collect();


        //? Finally, we use once again the magic of `filter_map` to keep only
        //? the bullets that are still alive.
        self.bullets = transition_bullets.into_iter()
            .filter_map(MaybeAlive::as_option)
            .collect();
        
        // TODO:
        // For the moment, we won't do anything about the player dying. This will be
        // the subject of a future episode.
        if !player_alive {
            println!("The player's ship has been destroyed.");
        }
        
        // Allow the player to shoot after the bullets are updated, so that,
        // when rendered for the first time, they are drawn wherever they
        // spawned.
        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }

        if ::rand::random::<usize>() % 100 == 0 {
            self.asteroids.push(self.asteroid_factory.random(phi));
        }
        
        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the Backgrounds
        self.bg_back.render(&mut phi.renderer, elapsed);
        self.bg_middle.render(&mut phi.renderer, elapsed);

        // Render the bounding box(for debugging purposes)
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl()).unwrap();
        }

        // Render the ship
        phi.renderer.copy_sprite(
            &self.player.sprites[self.player.current as usize],
            self.player.rect
        );

        // Render the bullets
        for bullet in &self.bullets {
            bullet.render(phi);
        }

        for asteroid in &self.asteroids {
            asteroid.render(phi);
        }

        for explosion in &self.explosions {
            explosion.render(phi);
        }

        // Render the foreground
        self.bg_front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}
