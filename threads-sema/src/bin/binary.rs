use std::sync::Arc;
use std::thread;
use std_semaphore::Semaphore;

static mut counter: i32 = 0;

fn child(sema: Arc<Semaphore>) {
    for i in 0..10000000 {
        sema.acquire();
        unsafe {
            counter += 1;
        }
        sema.release();
    }
}

fn main() {
    let mutex = Arc::new(Semaphore::new(1));
    let mutex2 = mutex.clone();
    let handle = thread::spawn(move || {
        child(mutex);
    });
    let handle2 = thread::spawn(move || {
        child(mutex2);
    });
    handle.join();
    handle2.join();
    unsafe {
        println!("result: {} (should be 20000000)", counter);
    }
}
