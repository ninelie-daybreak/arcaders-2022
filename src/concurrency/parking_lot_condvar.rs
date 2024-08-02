use std::sync::Arc;
use std::thread;

fn parking_lot_correct() {
    use parking_lot::{Condvar, Mutex};

    struct CondPair {
        lock: Mutex<bool>,
        cvar: Condvar,
    }

    impl CondPair {
        fn new() -> Self {
            Self {
                lock: Mutex::new(false),
                cvar: Condvar::new(),
            }
        }
        fn wait(&self) {
            let mut started = self.lock.lock();
            while !*started {
                self.cvar.wait(&mut started);
            }
        }
        fn notify(&self) {
            let mut started = self.lock.lock();
            *started = true;
            self.cvar.notify_one();
        }
    }

    let condvar1 = Arc::new(CondPair::new());
    let condvar2 = condvar1.clone();

    let th1 = thread::spawn(move || {
        condvar1.wait();
    });

    condvar2.notify();
    th1.join().unwrap();
}

fn parking_lot_deadlock_wait() {
    use parking_lot::{Condvar, Mutex};

    struct CondPair {
        lock: Mutex<bool>,
        cvar: Condvar,
        other: Mutex<i32>,
    }

    impl CondPair {
        fn new() -> Self {
            Self {
                lock: Mutex::new(false),
                cvar: Condvar::new(),
                other: Mutex::new(1),
            }
        }
        fn wait(&self) {
            let _i = self.other.lock();
            let mut started = self.lock.lock();
            while !*started {
                self.cvar.wait(&mut started);
            }
        }
        fn notify(&self) {
            let _i = self.other.lock();
            let mut started = self.lock.lock();
            *started = true;
            self.cvar.notify_one();
        }
    }

    let condvar1 = Arc::new(CondPair::new());
    let condvar2 = condvar1.clone();

    let th1 = thread::spawn(move || {
        condvar1.wait();
    });

    condvar2.notify();
    th1.join().unwrap();
}

fn parking_lot_missing_lock_before_notify() {
    use parking_lot::{Condvar, Mutex};

    struct CondPair {
        lock: Mutex<bool>,
        cvar: Condvar,
    }

    impl CondPair {
        fn new() -> Self {
            Self {
                lock: Mutex::new(false),
                cvar: Condvar::new(),
            }
        }
        fn wait(&self) {
            let mut started = self.lock.lock();
            while !*started {
                self.cvar.wait(&mut started);
            }
        }
        fn notify(&self) {
            self.cvar.notify_one();
        }
    }

    let condvar1 = Arc::new(CondPair::new());
    let condvar2 = condvar1.clone();

    let th1 = thread::spawn(move || {
        condvar1.wait();
    });

    condvar2.notify();
    th1.join().unwrap();
}