use std::{thread, time};

fn main() {
    let start = time::Instant::now();

    let handle = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });

    handle.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
