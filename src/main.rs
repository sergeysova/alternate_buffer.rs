extern crate alternate_buffer;

use std::thread;

fn main() {
    println!("Example output in main buffer");

    thread::sleep_ms(1000);
    alternate_buffer::show();

    thread::sleep_ms(1000);
    println!("Example output in alternate buffer");

    thread::sleep_ms(1000);
    alternate_buffer::hide();
}
