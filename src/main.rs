

use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let testString = b"Hello2";
    let width =24;
    let mut writer = BufWriter::new(stdout());

    say(testString, width, &mut writer).unwrap();
}
