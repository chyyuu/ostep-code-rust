use lazy_static::*;
use std::collections::VecDeque;
use std::env::args;
use std::io::{self, Write};
use std::process::exit;
use std::sync::Mutex;
use std::thread;
use std_semaphore::Semaphore;

lazy_static! {
    static ref BUFFER: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    static ref EMPTY: Semaphore = Semaphore::new(0);
    static ref FILL: Semaphore = Semaphore::new(0);
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 4 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n").unwrap();
        exit(1);
    }
    let max = args().nth(1).unwrap().parse::<i32>().unwrap();
    let loops = args().nth(2).unwrap().parse::<i32>().unwrap();
    let consumers = args().nth(3).unwrap().parse::<i32>().unwrap();
    for _ in 0..max {
        EMPTY.release();
    }

    let producer_handle = thread::spawn(move || {
        for i in 0..loops {
            EMPTY.acquire();
            let mut buffer2 = BUFFER.lock().unwrap();
            buffer2.push_back(i); // do_fill
            FILL.release();
        }

        // end case: put an end-of-production marker (-1)
        // into shared BUFFER, one per consumer
        for _ in 0..consumers {
            EMPTY.acquire();
            let mut buffer2 = BUFFER.lock().unwrap();
            buffer2.push_back(-1); // do_fill
            FILL.release();
        }
    });
    let mut consumer_handle = VecDeque::new();
    for _ in 0..consumers {
        let handle = thread::spawn(move || {
            let mut tmp = 0;
            while tmp != -1 {
                FILL.acquire();
                let mut buffer2 = BUFFER.lock().unwrap();
                tmp = buffer2.pop_front().unwrap(); // do get
                EMPTY.release();
            }
        });
        consumer_handle.push_back(handle);
    }
    producer_handle.join().unwrap();
    for handle in consumer_handle {
        handle.join().unwrap();
    }
}
