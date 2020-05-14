use pharosa::core::*;
use pharosa::scene::Scene;
use pharosa::primitive::{Primitive, Sphere, Material, bsdf, texture};
use pharosa::core::{Spectrum, Film};
use cgmath::{Matrix4, vec3};
use pharosa::camera::{Camera, Perspective, CameraInner};
use pharosa::sampler::{Independent, Fake};
use std::rc::Rc;
use pharosa::integrator::{self, Integrator};
use image::{ImageBuffer, Rgb, Pixel};

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
    let scene = setup_scene();
    let camera = setup_camera();
    let mut sampler = Fake;
    let mut film = Film::new(1024, 768);
    integrator::Position::render(&mut film, &camera, &scene, &mut sampler);
    let converted = ImageBuffer::from_fn(1024, 768, |x, y| {
        let pixel: RGBf = *film.get_pixel(x, y);
        Rgb::<u8>([
            (pixel.0[0] * 255.) as u8,
            (pixel.0[1] * 255.) as u8,
            (pixel.0[2] * 255.) as u8,
        ])
    });
    converted.save("result-pos.png").unwrap();
}
