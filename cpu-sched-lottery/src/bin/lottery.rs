extern crate rand;
use std::ptr::NonNull;
use std::env::args; 
use std::io::{self,Write};
use rand::prelude::*;
//use rand_pcg::Pcg64;
struct node_t{
    ticket:i32,
    next:  Option<NonNull<node_t>>
}

impl node_t{
    pub fn new(tickets:i32)->Self{
        Self{
          ticket:tickets,
          next : None
        }
    } 
}
struct linklist{
    head:Option<NonNull<node_t>>,
    gtickets:i32
}

impl linklist{
    pub fn new()->Self{
        linklist{
            head : None,
            gtickets:0
        }
    }
    fn insert(&mut self,ticket:i32){
        let mut node = node_t::new(ticket);
        let ptr = NonNull::<node_t>::new(&mut node as *mut _);
        if let Some(head) = self.head{
           node.next = unsafe{(*head.as_ptr()).next};
        }
          self.head = ptr;
          self.gtickets = self.gtickets + 1;

    }
}
fn print_list(L:&linklist){
    println!("List:");
    let mut curr = L.head;
    while let Some(p) = curr{
        unsafe{
        println!("[{}]",(*p.as_ptr()).ticket);
        curr =  (*p.as_ptr()).next;
        }
    }
}
fn main(){
    let argc = args().len();
    //println!("{:?}",argv.nth(2));
    let mut L = linklist::new();
    if argc != 3{
        let mut stderr = io::stderr();
        stderr.write(b"usage: lottery <seed> <loops>\n");
        std::process::exit(1);
    }
    else{
        println!("{:?}",args().nth(1));
        println!("{:?}",args().nth(2));
      let seed = args().nth(1).unwrap().parse::<i32>().unwrap();
      println!("{}",seed);
      let loops = args().nth(2).unwrap().parse::<i32>().unwrap();
      println!("{}",loops);
      L.insert(50);
      L.insert(100);
      L.insert(25);
      L.insert(200);
      print_list(&L);
    
      for i in 0..loops{
          let mut counter = 0;
          let winner = rand::random::<i32>()% (L.gtickets);
          let mut curr = L.head;
        while let Some(p) = curr{
        unsafe{
        counter = counter + (*p.as_ptr()).ticket;
        }
        if counter > winner{
            break;
        }
        unsafe{
        curr =  (*p.as_ptr()).next;
        }
        /*counter = counter + current->tickets;
	    if (counter > winner)
		break; // found the winner
	    current = current->next;*/
        }
        print_list(&L);
      }
    }
}

/*
int
main(int argc, char *argv[])
{
    if (argc != 3) {
	fprintf(stderr, "usage: lottery <seed> <loops>\n");
	exit(1);
    }
    int seed  = atoi(argv[1]);
    int loops = atoi(argv[2]);
    srandom(seed);

    // populate list with some number of jobs, each
    // with some number of tickets
    insert(50);
    insert(100);
    insert(25);

    print_list();
    
    int i;
    for (i = 0; i < loops; i++) {
	int counter            = 0;
	int winner             = random() % gtickets; // get winner
	struct node_t *current = head;

	// loop until the sum of ticket values is > the winner
	while (current) {
	    counter = counter + current->tickets;
	    if (counter > winner)
		break; // found the winner
	    current = current->next;
	}
	// current is the winner: schedule it...
	print_list();
	printf("winner: %d %d\n\n", winner, current->tickets);

    }
    return 0;
}
*/