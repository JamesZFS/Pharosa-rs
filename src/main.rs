#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Instant;

fn main() {
    let tic = Instant::now();
    pharosa::gui();
    println!("Program finished in {:?}", tic.elapsed());
}
