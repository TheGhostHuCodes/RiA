use std::{thread, time};

fn main() {
    let start = time::Instant::now();
    let pause = time::Duration::from_millis(300);

    let handle_1 = thread::spawn(move || {
        thread::sleep(pause);
    });

    let handle_2 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
