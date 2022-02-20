use lazy_static::*;
use std::env::args;
use std::io::{self, Write};
use std::process::*;
use std::sync::Arc;
use std::thread;
use std_semaphore::Semaphore;
use user_lib::*;

lazy_static! {
    static ref SEM: Arc<Semaphore> = Arc::new(Semaphore::new(0));
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 3 {
        let mut stderr = io::stderr();
        stderr
            .write(b"usage: throttle <num_threads> <sem_value>\n")
            .unwrap();
        exit(1);
    }
    let num_threads = args().nth(1).unwrap().parse::<isize>().unwrap();
    let sem_value = args().nth(2).unwrap().parse::<isize>().unwrap();
    for _ in 0..sem_value {
        SEM.release();
    }

    println!("parent: begin");
    let mut consumer_handle = Vec::new();
    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            // child
            SEM.acquire();
            println!("child {}", i);
            sleep(1);
            SEM.release();
        });
        consumer_handle.push(handle);
    }
    for handle in consumer_handle {
        handle.join().unwrap();
    }
    println!("parent: end");
}
