#![allow(unused_imports)]
#![allow(dead_code)]

use pharosa::core::*;
use pharosa::scene::Scene;
use pharosa::primitive::{Primitive, Sphere, Material, bsdf, texture};
use pharosa::core::{Spectrum, Film};
use cgmath::{Matrix4, vec3};
use pharosa::camera::{Camera, Perspective};
use pharosa::sampler::{Independent, Fake};
use std::rc::Rc;
use pharosa::integrator::*;
use std::time::Instant;
use pharosa::utils::ToImageBuffer;
use pharosa::gui::Gui;
use minifb::Key;


fn main() {
    let mut gui = Gui::new();
    gui.run();
    // let tic = Instant::now();
    // let scene = setup_scene();
    // let camera = setup_camera();
    // let mut sampler = Independent;
    // let mut film = Film::new(1024, 768);
    // let alg = SampleIntegrator { delegate: Normal, n_spp: 10 };
    // alg.render(&mut film, &camera, &scene, &mut sampler);
    // println!("Rendering done in {:?}", tic.elapsed());
    // film.to_image_buffer().save("result-normal.png").unwrap();
    // println!("Saved");
}
