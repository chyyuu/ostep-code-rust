use lazy_static::*;
use std::env::args;
use std::io::{self, Write};
use std::sync::Mutex;
use std::thread;
use std_semaphore::Semaphore;

lazy_static! {
    static ref FORKS: Vec<Semaphore> = vec![
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1)
    ];
    static ref PRINT_LOCK: Mutex<bool> = Mutex::new(true);
}

fn lock_print(s: String, space: i32) {
    let _lock = PRINT_LOCK.lock().unwrap();
    for _ in 0..space * 10 {
        print!(" ");
    }
    println!("{}", s);
}

fn think() {}

fn eat() {}

fn left(i: i32) -> usize {
    i as usize
}

fn right(i: i32) -> usize {
    ((i + 1) % 5) as usize
}

fn get_forks(i: i32) {
    if i == 4 {
        lock_print(format!("4: try {}", right(i)), i);
        FORKS[right(i)].acquire();
        lock_print(format!("4: try {}", left(i)), i);
        FORKS[left(i)].acquire();
    } else {
        lock_print(format!("{}: try {}", i, left(i)), i);
        FORKS[left(i)].acquire();
        lock_print(format!("{}: try {}", i, right(i)), i);
        FORKS[right(i)].acquire();
    }
}

fn put_forks(i: i32) {
    FORKS[right(i)].release();
    FORKS[left(i)].release();
}

fn philosopher(id: i32, num_loops: i32) {
    lock_print(format!("{}: start", id), id);
    for _ in 0..num_loops {
        lock_print(format!("{}: think", id), id);
        think();
        get_forks(id);
        lock_print(format!("{}: eat", id), id);
        eat();
        put_forks(id);
    }
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr
            .write(b"usage: dining_philosophers <num_loops>\n")
            .unwrap();
        std::process::exit(1);
    }
    let num_loops = args().nth(1).unwrap().parse::<i32>().unwrap();
    println!("dining: started");

    let mut handles = Vec::new();
    for i in 0..5 {
        handles.push(thread::spawn(move || philosopher(i as i32, num_loops)));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("dining: finished");
}
