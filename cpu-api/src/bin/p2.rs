extern crate  nix;
use nix::{unistd::*,sys::wait::*};
use std::ptr::null;
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
        sleep(1);
        }
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
     }
    /*int rc = fork();
    if (rc < 0) {
        // fork failed; exit
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        // child (new process)
        printf("hello, I am child (pid:%d)\n", (int) getpid());
	sleep(1);
    } else {
        // parent goes down this path (original process)
        int wc = wait(NULL);
        printf("hello, I am parent of %d (wc:%d) (pid:%d)\n",
	       rc, wc, (int) getpid());
    }
    return 0;*/
}


