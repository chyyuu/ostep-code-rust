extern crate nix;
use libc::c_void;
use libc::{pthread_create, pthread_join, pthread_t};
use std::env::args;
use std::io::{self, Write};
static mut counter: i32 = 0;
static mut loops: i32 = 0;
pub extern "C" fn worker(args: *mut c_void) -> *mut c_void {
    unsafe {
        for i in 0..loops {
            counter = counter + 1;
        }
    }
    0 as *mut c_void
}
fn main() {
    let mut argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n");
    } else {
        unsafe {
            loops = argv.nth(1).unwrap().parse::<i32>().unwrap();
        }
        let mut p1: pthread_t = 0;
        let mut p2: pthread_t = 0;
        unsafe {
            pthread_create(&mut p1, std::ptr::null(), worker, 0 as *mut c_void);
            pthread_create(&mut p2, std::ptr::null(), worker, 0 as *mut c_void);
            pthread_join(p1, 0 as *mut *mut c_void);
            pthread_join(p2, 0 as *mut *mut c_void);
        }
        unsafe {
            println!("Final value   : {}\n", counter);
        }
    }
}

/*
#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "common_threads.h"

volatile int counter = 0;
int loops;

void *worker(void *arg) {
    int i;
    for (i = 0; i < loops; i++) {
    counter++;
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
    fprintf(stderr, "usage: threads <loops>\n");
    exit(1);
    }
    loops = atoi(argv[1]);
    pthread_t p1, p2;
    printf("Initial value : %d\n", counter);
    Pthread_create(&p1, NULL, worker, NULL);
    Pthread_create(&p2, NULL, worker, NULL);
    Pthread_join(p1, NULL);
    Pthread_join(p2, NULL);
    printf("Final value   : %d\n", counter);
    return 0;
}
*/
