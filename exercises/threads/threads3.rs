// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
// use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {

    // make an clone of tx
    let tx_clone = tx.clone();

    // if compile seperately, the compiler will tell an error of q
/*
error[E0382]: use of moved value: `q`
  --> 1.rs:44:19
   |
28 | fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
   |            - move occurs because `q` has type `Queue`, which does not implement the `Copy` trait
...
36 |     thread::spawn(move || {
   |                   ------- value moved into closure here
37 |         for val in q.first_half {
   |                    ------------ variable moved due to use in closure
...
44 |     thread::spawn(move || {
   |                   ^^^^^^^ value used here after move
45 |         for val in q.second_half {
   |                    ------------- use occurs due to use in closure
*/

    thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

}


fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
