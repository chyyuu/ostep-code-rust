use intro::common::spin;
use std::env::args;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let mut argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n")?;
    } else {
        let argv1 = argv.nth(1).unwrap();
        loop {
            println!("{}", argv1);
            spin(1);
        }
    }
    Ok(())
}

/*
#include <stdio.h>
#include <stdlib.h>
#include "common.h"

int main(int argc, char *argv[])
{
    if (argc != 2) {
    fprintf(stderr, "usage: cpu <string>\n");
    exit(1);
    }
    char *str = argv[1];

    while (1) {
    printf("%s\n", str);
    Spin(1);
    }
    return 0;
}
*/
