use crate::*;
use minifb::*;
use std::time::Duration;
use std::sync::{Arc, RwLock};

mod config;

use config::*;
use crate::utils::ToHexColor;
use std::thread::spawn;

pub fn gui() {
    let window = setup_window();
    let context = Context {
        scene: setup_scene(),
        camera: setup_camera(),
        sampler: sampler::Independent,
        film: Arc::new(RwLock::new(Film::new(WIDTH, HEIGHT))),
    };
    let integrator = setup_integrator();
    event_loop(window, context, integrator);
}

fn event_loop(mut window: Window, mut context: Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>, integrator: impl Integrator) {
    // buffer for the window to display
    let mut buffer = Vec::with_capacity(context.film.read().unwrap().size());
    window.set_title(PHAROSA);

    let spp = Arc::new(RwLock::new(0u32));
    let film = context.film.clone();
    let kernel;
    { // create the kernel thread
        let spp = spp.clone();
        kernel = spawn(move || {
            println!("Kernel started.");
            // do rendering in 'kernel' thread
            loop {
                integrator.render(&mut context);
                *spp.write().unwrap() += 1;
            }
            println!("Kernel finished.");
        });
    }

    while window.is_open() {
        window.set_title(&format!("{} spp = {}", PHAROSA, spp.read().unwrap()));

        // refresh display
        buffer.clear();
        buffer.extend(film.read().unwrap().to_raw().iter().map(|s| s.to_hex_color()));
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();

        // process menu events
        // window.is_menu_pressed().map(|menu_id| {
        //     match menu_id {
        //         LOAD_SCENE_BTN => unimplemented!(),
        //         START_BTN => {
        //             println!("Start clicked!");
        //         }
        //         PAUSE_BTN => unimplemented!(),
        //         SAVE_BTN => unimplemented!(),
        //         _ => unreachable!()
        //     }
        // });
    }
    println!("Waiting for kernel to finish...");
    kernel.join().unwrap();
}

fn setup_window() -> Window {
    let mut window = Window::new
        (&format!("{} (initializing...)", PHAROSA), WIDTH as usize, HEIGHT as usize,
         WindowOptions {
             borderless: false,
             title: true,
             resize: true,
             scale: Scale::X1,
             scale_mode: ScaleMode::AspectRatioStretch,
             topmost: false,
         }).unwrap();
    window.limit_update_rate(Some(Duration::from_micros((1e6 / FPS_LIMIT) as u64))); // refresh rate
    let mut menu = Menu::new("Control").unwrap();
    menu.add_item("Load Scene", LOAD_SCENE_BTN).shortcut(Key::N, MENU_KEY_CTRL).build();
    menu.add_separator();
    menu.add_item("Start Rendering", START_BTN).shortcut(Key::Enter, MENU_KEY_CTRL).build();
    menu.add_item("Pause Rendering", PAUSE_BTN).shortcut(Key::Period, MENU_KEY_CTRL).build();
    menu.add_separator();
    menu.add_item("Save", SAVE_BTN).shortcut(Key::S, MENU_KEY_CTRL).build();
    window.add_menu(&menu);
    window.add_menu(&Menu::new("Help").unwrap());
    window
}

fn setup_scene() -> Scene<impl Geometry, impl BSDF, impl Texture> {
    let mut scene = Scene::new();
    scene.push(Primitive::new(
        Sphere::new(0.5),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform::new(Spectrum::new(1., 1., 0.)) }),
        Matrix4::from_translation(vec3(0., 0., 0.))));
    scene.push(Primitive::new(
        Sphere::new(2.),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform::new(Spectrum::new(0., 0., 1.)) }),
        Matrix4::from_translation(vec3(0., 0., 3.))));
    scene
}

fn setup_camera() -> Camera<impl CameraInner> {
    let pers = camera::Perspective::new(WIDTH, HEIGHT, FOVY);
    let camera = Camera::new(
        pers,
        Matrix4::look_at(
            pt3(0., 0., -4.),
            pt3(0., 0., 0.),
            vec3(0., 1., 0.)),
    );
    camera
}

fn setup_integrator() -> impl Integrator {
    integrator::SampleIntegrator { n_spp: 1, delegate: integrator::Normal }
}
