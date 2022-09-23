use crate::phi::Phi;
use crate::phi::data::Rectangle;
use sdl2::pixels::Color;

//? The velocity shared by all bullets, in pixels per second.
const BULLET_SPEED: f64 = 240.0;

//? The size of the rectangle which will represent the bullet.
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;

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

pub trait Bullet: {
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
pub enum CannonType {
    RectBullet,
    SineBullet { amplitude: f64, angular_vel: f64 },
    DevergentBullet { a: f64, b: f64},
}

pub fn spawn_bullets(cannon: CannonType, cannons_x: f64, cannons1_y: f64, cannons2_y: f64) -> Vec<Box<dyn Bullet>> {
    match cannon {
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
