#![feature(llvm_asm)]
static mut global:i32 = 0;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn compare_and_swap(ptr:*mut i32, old:i32, new:i32)->char {
    let mut ret:char = 'a'; 
    unsafe {
      llvm_asm!(" lock\n"
      " cmpxchgl %2,%1\n"
      " sete %0\n"
      : "=q" (ret), "=m" (*ptr),
      : "r" (new), "m" (*ptr), "a" (old)
      : "memory"
     );
    }
    ret
    }
    
    fn main() {
        unsafe{
        println!("before successful cas: {}", global);
        let success = compare_and_swap(&global as *const i32 as *mut i32, 0, 100);
        println!("after successful cas: {} (success: {})\n", global, success);
        
        println!("before failing cas: {}\n", global);
        success = compare_and_swap(&global as *const i32 as *mut i32, 0, 200);
        println!("after failing cas: {} (old: {})\n", global, success);
        }
    }