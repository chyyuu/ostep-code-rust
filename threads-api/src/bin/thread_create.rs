use std::thread;

#[derive(Debug)]
struct myarg_t {
    a: i32,
    b: i32,
}

fn mythread(args: myarg_t) {
    println!("{:?}", args);
}

fn main() {
    let args = myarg_t { a: 10, b: 20 };
    let handle = thread::spawn(|| mythread(args));
    handle.join().unwrap();
}
