use lazy_static::*;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

lazy_static!{
    static ref mState:Mutex<i32> = Mutex::new(0);
}

fn mMain() {
    println!("mMain: begin\n");
    let state = *mState.lock().unwrap();
    println!("mMain: state is {}", state);
}


fn main() {
    println!("ordering: begin");
    let mThread = thread::spawn(mMain);
    sleep(1);
    *mState.lock().unwrap() = 1;
    mThread.join().unwrap();
    println!("ordering: end\n");
}
