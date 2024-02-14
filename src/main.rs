

use circular_buffer::CircularBuffer;
use std::io::{BufWriter, stdout};


fn main() {
    let mut buf = CircularBuffer::<5, u32>::new();

    let testString = b"Hello2";

    let width =24;
    let mut writer = BufWriter::new(stdout());

    say(testString, width, &mut writer).unwrap();
}
