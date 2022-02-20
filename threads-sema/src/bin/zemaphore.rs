use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

struct Zem {
    cond: Condvar,
    lock: Mutex<i32>,
}

impl Zem {
    pub fn new(value: i32) -> Self {
        Zem {
            cond: Condvar::new(),
            lock: Mutex::new(value),
        }
    }

    pub fn wait(&self) {
        let mut lock = self.lock.lock().unwrap();
        while *lock <= 0 {
            lock = self.cond.wait(lock).unwrap();
        }
        *lock -= 1;
    }

    pub fn post(&self) {
        let mut lock = self.lock.lock().unwrap();
        *lock += 1;
        self.cond.notify_one();
    }
}

fn main() {
    let s = Arc::new(Zem::new(0));
    let s1 = s.clone();
    println!("parent: begin");
    thread::spawn(move || {
        sleep(4);
        println!("child");
        s1.post();
    });
    s.wait();
    println!("parent: end");
}
