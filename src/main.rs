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

fn setup_scene() -> Scene {
    let mut scene = Scene::new();
    scene.push(Primitive::new(
        Box::new(Sphere::new(0.5)),
        Rc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform::new(Spectrum::new(1., 1., 0.)) }),
        Matrix4::from_translation(vec3(0., 0., 0.))));
    scene.push(Primitive::new(
        Box::new(Sphere::new(2.)),
        Rc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform::new(Spectrum::new(0., 0., 1.)) }),
        Matrix4::from_translation(vec3(0., 0., 3.))));
    scene
}

fn setup_camera() -> Camera<Perspective> {
    let pers = Perspective::new(1024, 768, Deg(90.));
    let camera = Camera::new(
        pers,
        Matrix4::look_at(
            pt3(0., 0., -4.),
            pt3(0., 0., 0.),
            vec3(0., 1., 0.)),
    );
    camera
}

fn main() {
    let tic = Instant::now();
    let scene = setup_scene();
    let camera = setup_camera();
    let mut sampler = Independent;
    let mut film = Film::new(1024, 768);
    let alg = SampleIntegrator { delegate: Normal, n_spp: 10 };
    alg.render(&mut film, &camera, &scene, &mut sampler);
    println!("Rendering done in {:?}", tic.elapsed());
    film.to_image_buffer().save("result-normal.png").unwrap();
    println!("Saved");
}
