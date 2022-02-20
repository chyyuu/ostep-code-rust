use std::io::{self,Write};
use std::env::args;
use intro::common::Spin;


fn main() {
  let mut argv = args();
  let argc = argv.len();
  if argc != 2{
  let mut stderr = io::stderr();
  stderr.write(b"usage: cpu <string>\n");
  }
  else{
      let argv1 = argv.nth(1).unwrap();
      loop{
          println!("{}",argv1);
          Spin(1);
      }
  }
}
