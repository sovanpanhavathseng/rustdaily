//import the necessary modules
use std::thread;
use std::time::Duration;

fn main() {
   //create a new thread
   thread::spawn(|| {
      for i in 1..10 {
         println!("hi number {} from the spawned thread!", i);
         thread::sleep(Duration::from_millis(1));
      }
   });
   //code executed by the main thread
   for i in 1..5 {
      println!("hi number {} from the main thread!", i);
      thread::sleep(Duration::from_millis(1));
   }

   // one more thread in rust
   for i in 1..20 {
    println!("Second thread{} from the spawned thread", i);
    thread::sleep(Duration::from_millis(1));
   }

   // last thread
   for a in 1..10 {
    println!("last thread{} from the main",a);
    thread::sleep(Duration::from_millis(1));
   }
}
