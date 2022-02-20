use lazy_static::*;
use std::thread;
use std_semaphore::Semaphore;
use user_lib::*;

lazy_static! {
    static ref S: Semaphore = Semaphore::new(0);
}

fn child() {
    sleep(2);
    println!("child");
    S.release(); // signal here: child is done
}

fn main() {
    println!("parent: begin");
    thread::spawn(move || {
        child();
    });
    S.acquire();

    println!("parent: end");
}
