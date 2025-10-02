use std::io::{self, Write};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

pub struct SpinnerGuard {
    handle: Option<thread::JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
}

impl SpinnerGuard {
    pub fn new(message: &str) -> Self {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();
        let message = message.to_string();

        let handle = thread::spawn(move || {
            let frames = ["⠈", "⠐", "⠠", "⢀", "⡀", "⠄", "⠂", "⠁"];
            let mut i = 0;

            while !stop_flag_clone.load(Ordering::Relaxed) {
                print!("\r{} {}", frames[i], message);
                io::stdout().flush().unwrap();
                i = (i + 1) % frames.len();
                thread::sleep(Duration::from_millis(80));
            }

            // Clean up the line once stopped
            print!("\r{}\r", " ".repeat(message.len() + 4));
            io::stdout().flush().unwrap();
        });

        Self {
            handle: Some(handle),
            stop_flag,
        }
    }
}

impl Drop for SpinnerGuard {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
