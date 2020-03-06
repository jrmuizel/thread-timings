use std::time::Instant;

fn main() {
    let (s, r) = std::sync::mpsc::channel();
    //s.send(Instant::now());
    //r.recv().unwrap();
    let j = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        s.send(Instant::now()).unwrap();
    } );
    let start = r.recv().unwrap();

    let time = Instant::now() - start;
    let start = Instant::now();
    let measure = Instant::now() - start;
    println!("Thread switch timing {} {}", time.as_secs_f32() * 1000., measure.as_secs_f32() * 1000.);
}
