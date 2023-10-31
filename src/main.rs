mod algos;

use crate::algos::bigo;

fn main() {
    let sum_chars = bigo::sum_char_codes("Hello world");

    println!("bigo::sum_char_codes() {}", sum_chars);
}


