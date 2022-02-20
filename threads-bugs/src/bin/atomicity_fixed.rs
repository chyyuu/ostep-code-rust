use lazy_static::*;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

#[derive(Debug)]
struct thread_info_t {
    pid: i32,
}

lazy_static! {
    static ref thd: Mutex<Option<thread_info_t>> = Mutex::new(Some(thread_info_t { pid: 100 }));
}

fn thread1() {
    println!("t1: before check");
    let thd_lock = thd.lock().unwrap();
    println!("t1: {:?}", *thd_lock);
    println!("t1: after check");
    sleep(2);
    println!("t1: use!");
    println!("t1: {:?}", *thd_lock);
}

fn thread2() {
    println!("                 t2: begin");
    sleep(1);
    let mut thd_lock = thd.lock();
    println!("                 t2: set to NULL");
    *thd_lock.unwrap() = None;
}

fn main() {
    println!("main: begin");
    let handle1 = thread::spawn(|| thread1());
    let handle2 = thread::spawn(|| thread2());
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("main: end");
}
