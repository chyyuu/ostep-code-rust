use lazy_static::*;
use std::mem::drop;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

lazy_static!{
    static ref mState:Mutex<i32> = Mutex::new(0);
}

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    println!("ordering: begin");
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("mMain: state is {}", *mState.lock().unwrap());
    });
    sleep(1);
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *mState.lock().unwrap() = 1;
    *started = true;
    cvar.notify_one();
    drop(started); // unlock
    handle.join().unwrap();
    println!("ordering: end");
}
