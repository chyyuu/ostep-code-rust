use gag::Redirect;
use std::ffi::CStr;
use std::fs::OpenOptions;
use user_lib::*;

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child: _, .. }) => {
            if let Ok(wc) = wait() {
                if let WaitStatus::Exited(pid, _exitcode) = wc {
                    assert!(pid.as_raw() >= 0);
                }
            }
        }
        Ok(ForkResult::Child) => {
            let log = OpenOptions::new()
                .truncate(true)
                .read(true)
                .create(true)
                .write(true)
                .open("./p4.output")
                .unwrap();
            let _print_redirect = Redirect::stdout(log).unwrap();
            let arg0 = CStr::from_bytes_with_nul(b"wc\0").unwrap();
            let arg1 = CStr::from_bytes_with_nul(b"p4.rs\0").unwrap();
            let myargs: [&CStr; 2] = [&arg0, &arg1];
            execvp(myargs[0], &myargs).unwrap();
        }
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
    }
}