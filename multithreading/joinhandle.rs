use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!(" Hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!(" Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1))
    }

    // add one more loop
    for a in 1..5 {
        println!(" Hi number {} from the loop a", a);

        thread::sleep(Duration::from_millis(1));
    }

    // add one more loop
    for b in 1..5 {
        println!(" Hi number {} from the loop b", b);

        thread::sleep(Duration::from_millis(1));
    } 

     // add one more loop
     for c in 1..5 {
        println!(" Hi number {} from the loop c", c);

        thread::sleep(Duration::from_millis(1));
    }

     // add one more loop
     for d in 1..5 {
        println!(" Hi number {} from the loop d", d);

        thread::sleep(Duration::from_millis(1));
    }
    // stand handle join
    handle.join().unwrap();
}
