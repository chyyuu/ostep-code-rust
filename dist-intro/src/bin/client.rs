use std::net::UdpSocket;
use std::str::from_utf8;
fn main(){
   
   let socket = UdpSocket::bind("0.0.0.0:20000").unwrap();

   let message = "hello world".as_bytes();
   println!("client:: send message [{}]",from_utf8(message).unwrap());
   let rc = socket.send_to(&message, "0.0.0.0:10000").unwrap();
   println!("client:: wait for reply...\n");
   let mut message2 = [0;1000];
   let rc = socket.recv_from(&mut message2).unwrap().0;
   println!("client:: got reply [size:{} contents:({})",rc,from_utf8(message).unwrap());
}
