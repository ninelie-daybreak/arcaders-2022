use std::sync::Arc;
use std::thread;

fn std_correct() {
    use std::sync::{Condvar, Mutex};

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
            let mut started = self.lock.lock().unwrap();
       
            while !*started {
                started = self.cvar.wait(started).unwrap();
            }
        }
        fn notify(&self) {
            let mut started = self.lock.lock().unwrap();

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

fn std_deadlock_wait() {
    use std::sync::{Condvar, Mutex};

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
            let _i = self.other.lock().unwrap();
            let mut started = self.lock.lock().unwrap();
            while !*started {
                started = self.cvar.wait(started).unwrap();
            }

        }
        fn notify(&self) {
            let _i = self.other.lock().unwrap();
            let mut started = self.lock.lock().unwrap();

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

fn std_missing_lock_before_notify() {
    use std::sync::{Condvar, Mutex};

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
            let mut started = self.lock.lock().unwrap();

            while !*started {
                started = self.cvar.wait(started).unwrap();
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

