use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

mod ticker;
mod reader;

fn main() {
    // Mutex is used so that count can be mutated while allowing the closure to implement `Fn()` instead of `FnMut()`.
    let count = Arc::new(Mutex::new(0));
    let count_borrow = Arc::clone(&count);

    // ticker function block the main thread by running the infinite loop.
    // ticker::ticker(move || {
    //     // The closure borrows count by moving it into the closure with move.
    //     // Inside the closure, we lock the mutex to safely access and update the value of count.
    //     let mut current = count.lock().unwrap();
    //     println!("{}", *current);
    //     *current += 1;
    // });

    let counter_closure =  move || {
        // The closure borrows count by moving it into the closure with move.
        // Inside the closure, we lock the mutex to safely access and update the value of count.
        let mut current = count_borrow.lock().unwrap();
        println!("{}", *current);
        *current += 1;
    };
    let ticker = Arc::new(ticker::Ticker::new(counter_closure));

    ticker.start();
    sleep(Duration::from_secs(10));

    ticker.stop();

    println!("Final value {}", count.lock().unwrap());
}
