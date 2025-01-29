use std::sync::Mutex;

mod ticker;

fn main() {
    // Mutex is used so that count can be mutated while allowing the closure to implement `Fn()` instead of `FnMut()`.
    let count = Mutex::new(0);

    // ticker function block the main thread by running the infinite loop.
    ticker::ticker(move || {
        // The closure borrows count by moving it into the closure with move.
        // Inside the closure, we lock the mutex to safely access and update the value of count.
        let mut current = count.lock().unwrap();
        println!("{}", *current);
        *current += 1;
    });
}
