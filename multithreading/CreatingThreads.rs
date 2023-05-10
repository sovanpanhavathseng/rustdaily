// Now that we understand the different approaches for the concurrency in Rust. creating threads is the most basic one-

use std::threads;


fn main(){
  println!("Before the thread!");

  let handle = thread::spawn(|| {
    println!("Inside the threads!");

};

  println!("After the thread spawn!);
  handle.join().expect("The thread panicked");
  println!("After everything");
}
