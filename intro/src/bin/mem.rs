extern crate nix;
use intro::common::spin;
use nix::unistd::{getppid};
use std::alloc::{alloc, Layout};
use std::env::args;
use std::io::{self, Write};
fn main() -> std::io::Result<()> {
    let argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n")?;
        std::process::exit(1);
    } else {
        let layout = Layout::new::<i32>();
        let p = unsafe { alloc(layout) };
        //assert_ne!(p,std::ptr::null());
        println!(
            "({}) addr pointed to by p: {}\n",
            getppid().as_raw(),
            p as usize
        );
        unsafe {
            *(p as *mut i32) = args().nth(1).unwrap().parse::<i32>().unwrap();
        }
        loop {
            spin(1);
            unsafe {
                *(p as *mut i32) = *(p as *mut i32) + 1;
            }
            println!("({}) value of p: {}\n", getppid().as_raw(), unsafe {
                *(p as *mut i32)
            });
        }
    }
}

/*
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include "common.h"

int main(int argc, char *argv[]) {
    if (argc != 2) {
    fprintf(stderr, "usage: mem <value>\n");
    exit(1);
    }
    int *p;
    p = malloc(sizeof(int));
    assert(p != NULL);
    printf("(%d) addr pointed to by p: %p\n", (int) getpid(), p);
    *p = atoi(argv[1]); // assign value to addr stored in p
    while (1) {
    Spin(1);
    *p = *p + 1;
    printf("(%d) value of p: %d\n", getpid(), *p);
    }
    return 0;
}
*/
