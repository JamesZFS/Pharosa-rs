#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Instant;

fn bench_1() {
    use pharosa::*;
    let a = Spectrum::black();
    let b = Spectrum::white();
    let n = 10000000;
    for i in 0..n {
        let c = lerp(&a, &b, i as Float / n as Float);
        if i % 1000000 == 0 { println!("{:?}", c); }
    }
}

fn bench_2() {
    use pharosa::*;
    let a = Spectrum::black();
    let b = Spectrum::white();
    let n = 10000000;
    for i in 0..n {
        let c = &a + (&b - &a) * (i as Float / n as Float);
        if i % 1000000 == 0 { println!("{:?}", c); }
    }
}

fn main() {
    println!("Pharosa {}\n", env!("CARGO_PKG_VERSION"));
    let tic = Instant::now();
    // bench_1();
    // let e1 = tic.elapsed();
    // let tic = Instant::now();
    // bench_2();
    // let e2 = tic.elapsed();
    // println!("{:?} vs {:?}", e1, e2);
    pharosa::gui();
    println!("\nProgram finished in {:?}", tic.elapsed());
}
