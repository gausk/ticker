use std::thread::{self, sleep};
use std::time::Duration;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};



pub struct Ticker<T: Fn() + Send + 'static>{
    function: T,
    running: AtomicBool,
}

impl<T: Fn() + Send + Sync + 'static>  Ticker<T> {
    pub fn start(self: &Arc<Self>) {
        let ticker = Arc::clone(self);
        if ticker.running.load(Ordering::Relaxed) {
            return;
        }
        ticker.running.store(true, Ordering::Relaxed);
        thread::spawn(move || {
            while ticker.running.load(Ordering::Relaxed) {
                (ticker.function)();
                sleep(Duration::from_secs(1));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn new(function: T) -> Ticker<T> {
        Self {
            function,
            running: AtomicBool::new(false),
        }
    }
}



pub fn ticker<F>(function: F)
where
    F: Fn() + Send + 'static,
{
    loop {
        function();
        sleep(Duration::from_secs(1));
    }
}
