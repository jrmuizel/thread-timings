use std::time::Instant;
use std::{sync, thread};

fn main() {
    let (s, r) = std::sync::mpsc::channel();
    let j = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        s.send(Instant::now()).unwrap();
    } );

    let start = r.recv().unwrap();
    let time = Instant::now() - start;

    let start = Instant::now();
    let precision = Instant::now() - start;
    println!("Thread switch timing {}us {}us", time.as_secs_f32() * 1000.*1000., precision.as_secs_f32() * 1000.*1000.);
    
    j.join().unwrap();
}
