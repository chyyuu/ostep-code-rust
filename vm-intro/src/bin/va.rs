use std::alloc::{System,Layout,alloc};
use std::num::NonZeroUsize;
fn main(){
    
    println!("location of code : {}\n", main as usize);
    let layout = Layout::new::<[u8;10000]>();
    let ptr = unsafe{alloc(layout)};
    println!("location of heap : {}\n", ptr as usize);
    let x = 3;
    let ptr = &x as *const i32 as usize;
    println!("location of heap : {}\n", ptr);
    //int x = 3;
    //printf("location of stack: %p\n", &x);
} 