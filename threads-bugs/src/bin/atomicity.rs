use std::thread;
use user_lib::*;

#[derive(Debug)]
struct thread_info_t {
    pid: i32,
}

static mut thd: Option<thread_info_t> = Some(thread_info_t { pid: 100 });

fn thread1() {
    println!("t1: before check");
    unsafe {
        println!("t1: {:?}", thd);
        println!("t1: after check");
        sleep(2);
        println!("t1: use!");
        println!("t1: {:?}", thd);
    }
}

fn thread2() {
    println!("                 t2: begin");
    sleep(1);
    unsafe {
        println!("                 t2: set to NULL");
        thd = None;
    }
}

fn main() {
    println!("main: begin");
    let handle1 = thread::spawn(|| thread1());
    let handle2 = thread::spawn(|| thread2());
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("main: end");
}
