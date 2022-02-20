extern crate  nix;
use std::io::{self,Write};
use std::env::args;
use intro::common::Spin;
use std::alloc::{System,Layout,alloc};
use nix::unistd::{Pid,getppid};
fn main(){
  let mut argv = args();
  let argc = argv.len();
  if argc != 2{
  let mut stderr = io::stderr();
  stderr.write(b"usage: cpu <string>\n");
  std::process::exit(1);
  }
  else{
    let layout = Layout::new::<i32>();
    let mut p = unsafe{alloc(layout)}; 
    //assert_ne!(p,std::ptr::null());
    println!("({}) addr pointed to by p: {}\n", getppid().as_raw(), p as usize);
    unsafe{
    *(p as *mut i32) = args().nth(1).unwrap().parse::<i32>().unwrap();
    }
    loop{
        Spin(1);
        unsafe{
            *(p as *mut i32) = *(p as *mut i32) + 1;
        }
        println!("({}) value of p: {}\n", getppid().as_raw(),unsafe{ *(p as *mut i32)});
    }
}
}

