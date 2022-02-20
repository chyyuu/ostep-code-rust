use std::thread;

#[derive(Debug)]
struct myarg_t {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct myret_t {
    x: i32,
    y: i32,
}

fn mythread(args: myarg_t) -> myret_t {
    println!("{:?}", args);
    return myret_t {
        x: args.a,
        y: args.b,
    };
}

fn main() {
    let args = myarg_t { a: 10, b: 20 };
    let handle = thread::spawn(|| mythread(args));
    let rvals = handle.join().unwrap();
    println!("{:?}", rvals);
}
