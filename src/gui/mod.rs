use crate::*;
use minifb::*;
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::utils::{ToHexColor, ToImageBuffer, GammaCorrection};
use std::thread::spawn;

mod config;

use config::*;

pub fn gui() {
    let window = setup_window();
    let context = Context {
        scene: setup_scene(),
        camera: setup_camera(),
        sampler: sampler::Independent,
        film: Arc::new(RwLock::new(Film::new(WIDTH, HEIGHT))),
        progress: Arc::new(Default::default()),
    };
    let integrator = setup_integrator();
    event_loop(window, context, integrator);
}

fn event_loop(mut window: Window, mut context: Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>, integrator: impl Integrator) {
    // buffer for the window to display
    let mut buffer = Vec::with_capacity(context.film.read().unwrap().size());
    window.set_title(PHAROSA);

    let progress = context.progress.clone();
    let kernel_is_done = Arc::new(AtomicBool::new(false));
    let film = context.film.clone();
    let kernel;
    { // spawn the kernel thread
        let kernel_is_done = kernel_is_done.clone();
        // do rendering in 'kernel' thread
        kernel = spawn(move || {
            println!("Kernel started.");
            #[cfg(debug_assertions)] {
                println!("{:#?}", context);
                println!("{:#?}", integrator);
            }
            let tic = Instant::now();
            integrator.render(&mut context);
            println!("Kernel finished in {:?}", tic.elapsed());
            kernel_is_done.store(true, Ordering::Relaxed);
        });
    }

    while window.is_open() {
        if kernel_is_done.load(Ordering::Relaxed) {
            window.set_title(&format!("{} (done)", PHAROSA));
        } else {
            window.set_title(&format!("{} (rendering {:.1}% ...)", PHAROSA, *progress.read().unwrap() * 100.));
        }

        // refresh display
        update_buffer(&mut buffer, &film);
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();

        // process menu events
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                LOAD_SCENE_BTN => unimplemented!(),
                START_BTN => {
                    println!("Start clicked!");
                }
                PAUSE_BTN => unimplemented!(),
                SAVE_BTN => save(&film),
                _ => unreachable!()
            }
        });
    }
    println!("Waiting for kernel to finish...");
    kernel.join().unwrap();
}

fn update_buffer(buffer: &mut Vec<u32>, film: &Arc<RwLock<Film>>) {
    buffer.clear();
    buffer.extend(film.read().unwrap().to_raw().iter().map(|s| s.gamma_correction().to_hex_color()));
}

fn save(film: &Arc<RwLock<Film>>) {
    film.read().unwrap().to_image_buffer().save(SAVE_PATH).unwrap();
    println!("Result saved to '{}'", SAVE_PATH);
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
             topmost: true,
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

///
/// Sphere spheres[] = {//Scene: radius, position, emission, color, material
//    Sphere(1e5, Vec( 1e5+1,40.8,81.6), Vec(),Vec(.75,.25,.25),DIFF),//Left
//    Sphere(1e5, Vec(-1e5+99,40.8,81.6),Vec(),Vec(.25,.25,.75),DIFF),//Rght
//    Sphere(1e5, Vec(50,40.8, 1e5),     Vec(),Vec(.75,.75,.75),DIFF),//Back
//    Sphere(1e5, Vec(50,40.8,-1e5+170), Vec(),Vec(),           DIFF),//Frnt
//    Sphere(1e5, Vec(50, 1e5, 81.6),    Vec(),Vec(.75,.75,.75),DIFF),//Botm
//    Sphere(1e5, Vec(50,-1e5+81.6,81.6),Vec(),Vec(.75,.75,.75),DIFF),//Top
//    Sphere(16.5,Vec(27,16.5,47),       Vec(),Vec(1,1,1)*.999, SPEC),//Mirr
//    Sphere(16.5,Vec(73,16.5,78),       Vec(),Vec(1,1,1)*.999, REFR),//Glas
//    Sphere(600, Vec(50,681.6-.27,81.6),Vec(12,12,12),  Vec(), DIFF) //Lite
//  };

fn setup_scene() -> Scene<impl Geometry, impl BSDF, impl Texture> {
    let radius = [1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 16.5, 16.5, 600.];
    let position = [
        [1e5 + 1., 40.8, 81.6],
        [-1e5 + 99., 40.8, 81.6],
        [50., 40.8, 1e5],
        [50., 40.8, -1e5 + 170.],
        [50., 1e5, 81.6],
        [50., -1e5 + 81.6, 81.6],
        [27., 16.5, 47.],
        [73., 16.5, 78.],
        [50., 681.6 - 0.27, 81.6],
    ];
    let emission = [
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [0., 0., 0.],
        [12., 12., 12.],
    ];
    let color = [
        [0.75, 0.25, 0.25],
        [0.25, 0.25, 0.75],
        [0.75, 0.75, 0.75],
        [0., 0., 0.],
        [0.75, 0.75, 0.75],
        [0.75, 0.75, 0.75],
        [0.999, 0.999, 0.999],
        [0.999, 0.999, 0.999],
        [0., 0., 0.],
    ];
    use bsdf::simple::*;
    let mater = [
        Simple::Diffuse(Diffuse),
        Diffuse.into(),
        Diffuse.into(),
        Diffuse.into(),
        Diffuse.into(),
        Diffuse.into(),
        Specular.into(),
        Dielectric::default().into(),
        Diffuse.into(),
    ];

    let mut scene = Scene::new();
    for i in 0..radius.len() {
        scene.push(Primitive::new(
            Sphere::new(radius[i]),
            Arc::new(Material {
                bsdf: mater[i].clone(),
                texture: texture::Uniform(Spectrum::new(color[i][0], color[i][1], color[i][2])),
                emission: Spectrum::new(emission[i][0], emission[i][1], emission[i][2]),
            }),
            Matrix4::from_translation(position[i].into())
        ))
    };
    scene
}

fn setup_camera() -> Camera<impl CameraInner> {
    let pers = camera::Perspective::new(WIDTH, HEIGHT, Deg(40.));
    let camera = Camera::new(
        pers,
        Matrix4::look_at(
            pt3(50., 52., 295.6),
            pt3(50., 52., 0.),
            vec3(0., -1., 0.)),
    );
    camera
}

fn setup_integrator() -> impl Integrator {
    // integrator::SampleIntegrator { n_spp: 100, delegate: integrator::SmallPT { rr_depth: 5 } }
    integrator::SampleIntegrator { n_spp: 1, delegate: integrator::Albedo::default() }
}
