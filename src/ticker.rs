use std::thread::sleep;
use std::time::Duration;

pub fn ticker<F>(function: F)
where
    F: Fn() + Send + 'static,
{
    loop {
        function();
        sleep(Duration::from_secs(1));
    }
}
