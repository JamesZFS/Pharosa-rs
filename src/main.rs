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

fn test_onb() {
    use pharosa::*;
    let ez = vec3(0., 0., 1.);
    let onb = onb(ez);
    println!("{:?}", onb);
    let v = onb * vec3(0.1, 0.2, 1.);
    println!("{:?}", v);
}

fn sum_of_squares(input: &[i32]) -> i32 {
    use rayon::prelude::*;
    input.par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn main() {
    println!("Pharosa {}\n", env!("CARGO_PKG_VERSION"));
    let tic = Instant::now();
    // test_sample();
    pharosa::gui();
    // println!("{:?}", sum_of_squares(&(0..100000000).into_iter().collect::<Vec<_>>()));
    // test_onb();
    println!("\nProgram finished in {:?}", tic.elapsed());
}
