use lazy_static::*;
use std::mem::drop;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

lazy_static!{
static ref L1:Mutex<u8> = Mutex::new(1);
static ref L2:Mutex<u8> = Mutex::new(1);
}
fn thread1(){
    println!("t1:  begin");
    println!("t1: try to acquire L1...");
    let l1 = L1.lock().unwrap();
    sleep(1);
    println!("t1: L1 acquired");
    println!("t1: try to acquire L2...");
    let l2 = L2.lock().unwrap();
    println!("t1: L2 acquired");
    drop(l1);
    drop(l2);
}
fn thread2(){
    println!("                           t2: begin");
    println!("                           t2: try to acquire L2...");
    let l2 = L2.lock().unwrap();
    println!("                           t2: L2 acquired");
    println!("                           t2: try to acquire L1..");
    let l1 = L1.lock().unwrap();
    println!("                           t2: L1 acquired");
    drop(l1);
    drop(l2);
}
fn main(){
      println!("main: begin");
        let p1 = thread::spawn(||{
            thread1();
        });
        let p2 = thread::spawn(||{
            thread2();
        });
        p1.join().unwrap();
        p2.join().unwrap();
      println!("main: end");
    
}


