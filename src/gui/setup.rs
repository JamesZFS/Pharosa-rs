#![allow(dead_code)]

use super::*;

pub fn setup_window() -> Window {
    let mut window = Window::new(&format!("{} (initializing...)", PHAROSA), WIDTH as usize, HEIGHT as usize, WINDOW_OPTS)
        .unwrap();
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

pub fn setup_scene_cornell() -> Scene<impl Geometry, impl BSDF, impl Texture> {
    let radius = [1e5, 1e5, 1e5, 1e5, 1e5, 1e5, 16.5, 16.5, 600.];
    let position = [
        [1e5 + 1., 40.8, 81.6],
        [-1e5 + 99., 40.8, 81.6],
        [50., 40.8, 1e5],
        [50., 40.8, -1e5 + 370.],
        [50., 1e5, 81.6],
        [50., -1e5 + 81.6, 81.6],
        [27., 16.5, 47.],
        [73., 16.5, 78.],
        [50., 681.6 - 0.27, 81.6], // Light
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
            Matrix4::from_translation(position[i].into()),
        ))
    };
    scene
}

pub fn setup_camera_cornell() -> Camera<impl CameraInner> {
    let pers = camera::Perspective::new(WIDTH, HEIGHT, Deg(30.));
    let camera = Camera::new(
        pers,
        pt3(50., 52., 295.6),
        pt3(50., 40., 0.),
        vec3(0., 1., 0.),
    );
    camera
}

pub fn setup_scene() -> Scene<impl Geometry, impl BSDF, impl Texture> {
    let mut scene = Scene::new();
    scene.push(Primitive::new(
        Sphere::new(0.3),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform(Spectrum::new(1., 0., 0.)), emission: Spectrum::new(0.5, 0.2, 0.5) }),
        Matrix4::from_translation(vec3(3., 0., 0.))));
    scene.push(Primitive::new(
        Sphere::new(0.3),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform(Spectrum::new(0., 1., 0.)), emission: Spectrum::new(0.5, 0.2, 0.5) }),
        Matrix4::from_translation(vec3(0., 3., 0.))));
    scene.push(Primitive::new(
        Sphere::new(0.3),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform(Spectrum::new(0., 0., 1.)), emission: Spectrum::new(0.2, 0.5, 0.2) }),
        Matrix4::from_translation(vec3(0., 0., 3.))));
    scene.push(Primitive::new(
        Sphere::new(0.1),
        Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform(Spectrum::new(1., 1., 1.)), emission: Spectrum::new(0.5, 0.2, 0.5) }),
        Matrix4::from_translation(vec3(0., 0., 0.))));
    scene
}

pub fn setup_camera() -> Camera<impl CameraInner> {
    let pers = camera::Perspective::new(WIDTH, HEIGHT, Deg(90.));
    let camera = Camera::new(
        pers,
            pt3(0., 0., -2.),
            pt3(0., 0., 0.),
            vec3(0., 1., 0.),
    );
    camera
}

pub fn setup_integrator() -> impl Integrator {
    integrator::SampleIntegrator { n_spp: 100, delegate: integrator::SmallPT { rr_depth: 5 } }
    // integrator::SampleIntegrator { n_spp: 10, delegate: integrator::Normal::default() }
}
