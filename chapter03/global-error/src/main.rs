extern crate rand;

use rand::{thread_rng, Rng};

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: Vec<u8>) -> usize {
    // if thread_rng().gen_bool(1.0 / 10_000.0) {
    if thread_rng().gen_bool(1.0) {
        unsafe {
            ERROR = 1;
        }
    }
    0
}
#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];
    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
