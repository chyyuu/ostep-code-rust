extern crate nix;
extern crate libc;
use nix::{unistd::*,sys::wait::*};
use std::ptr::null;
use std::ffi::CStr;
fn main()
{
    println!("hello world (pid:{})", getpid());
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            if let Ok(wc) = wait(){
            if let WaitStatus::Exited(pid,exitcode) = wc{
            println!("hello, I am parent of {}(wc:{}) (pid:{})", child, pid,getpid());
            }
            }
        }
        Ok(ForkResult::Child) => {
        println!("hello, I am child (pid:{})",getpid());
        let arg0 = CStr::from_bytes_with_nul(b"wc\0").unwrap();
        let arg1 = CStr::from_bytes_with_nul(b"p3.rs\0").unwrap();
        let arg2 = CStr::from_bytes_with_nul(b"\0").unwrap();
        let myargs:[&CStr;3]=[&arg0,&arg1,&arg2];
        execvp(myargs[0],&myargs);
        sleep(1);
        }
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
     }
  
    }