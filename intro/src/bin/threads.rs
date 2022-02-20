extern crate nix;
use libc::{pthread_create,pthread_join,pthread_t};
use libc::c_void;
use std::io::{self,Write};
use std::env::args;
static mut counter:i32 = 0;
static mut loops:i32 = 0;
pub extern "C" fn worker(args:*mut c_void)->*mut c_void{
    unsafe{
    for i in 0..loops{
      counter = counter + 1;
    }
}
    0 as *mut c_void
}
fn main(){
    let mut argv = args();
    let argc = argv.len();
    if argc != 2{
    let mut stderr = io::stderr();
    stderr.write(b"usage: cpu <string>\n");
    }
    else{
    unsafe{
     loops= argv.nth(1).unwrap().parse::<i32>().unwrap();
    }
    let mut p1:pthread_t = 0;
    let mut p2:pthread_t = 0;
    unsafe{
    pthread_create(&mut p1,std::ptr::null(),worker,0 as *mut c_void);
    pthread_create(&mut p2,std::ptr::null(),worker,0 as *mut c_void);
    pthread_join(p1,0 as *mut *mut c_void);
    pthread_join(p2,0 as *mut *mut c_void);
    }
    unsafe{
    println!("Final value   : {}\n", counter);
    }
}
}


