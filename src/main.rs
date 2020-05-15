#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Instant;

fn main() {
    println!("Pharosa {}\n", env!("CARGO_PKG_VERSION"));
    let tic = Instant::now();
    pharosa::gui();
    println!("\nProgram finished in {:?}", tic.elapsed());
}
