use lazy_static::*;
use std::env::args;
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread;
use std_semaphore::Semaphore;

static mut COUNTER: i32 = 0;

lazy_static! {
    static ref LOCK: Mutex<Rwlock> = Mutex::new(Rwlock::new());
}

struct Rwlock {
    writelock: Semaphore,
    lock: Semaphore,
    readers: i32,
}

impl Rwlock {
    pub fn new() -> Self {
        Rwlock {
            writelock: Semaphore::new(1),
            lock: Semaphore::new(1),
            readers: 0,
        }
    }

    pub fn acquire_readlock(&mut self) {
        self.lock.acquire();
        self.readers += 1;
        if self.readers == 1 {
            self.writelock.acquire();
        }
        self.lock.release();
    }

    pub fn release_readlock(&mut self) {
        self.lock.acquire();
        self.readers -= 1;
        if self.readers == 0 {
            self.writelock.release();
        }
        self.lock.release();
    }

    pub fn acquire_writelock(&self) {
        self.writelock.acquire();
    }

    pub fn release_writelock(&self) {
        self.writelock.release();
    }
}

fn reader(read_loops: i32) {
    let mut local: i32 = 0;
    for _i in 0..read_loops {
        let mut lock1 = LOCK.lock().unwrap();
        lock1.acquire_readlock();
        unsafe {
            local = COUNTER;
        }
        lock1.release_readlock();
        println!("read {}", local);
    }
    println!("read done: {}", local);
}

fn writer(write_loops: i32) {
    for _i in 0..write_loops {
        let lock1 = LOCK.lock().unwrap();
        lock1.acquire_writelock();
        unsafe {
            COUNTER = COUNTER + 1;
        }
        lock1.release_writelock();
    }
    println!("write done");
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 3 {
        let mut stderr = io::stderr();
        stderr
            .write(b"usage: rwlock readloops writeloops\n")
            .unwrap();
        std::process::exit(1);
    }
    let read_loops = args().nth(1).unwrap().parse::<i32>().unwrap();
    let write_loops = args().nth(2).unwrap().parse::<i32>().unwrap();
    let handle = thread::spawn(move || {
        reader(read_loops);
    });
    let handle2 = thread::spawn(move || {
        writer(write_loops);
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("all done");
}
