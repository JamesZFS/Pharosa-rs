use crate::*;
use minifb::*;
use std::time::Duration;

mod config;

use config::*;
use std::sync::Arc;

pub struct Gui {
    /// buffer is always the u32 repr of film
    buffer: Vec<u32>,
    window: Window,
}

impl Gui {
    pub fn new() -> Self {
        let film = Film::new(WIDTH, HEIGHT);
        let scene = setup_scene();
        let buffer = vec![0xffffffu32; film.size()];
        let mut window = Window::new(WINDOW_TITLE, film.width() as usize, film.height() as usize,
                                     WindowOptions {
                                         borderless: false,
                                         title: true,
                                         resize: true,
                                         scale: Scale::X1,
                                         scale_mode: ScaleMode::AspectRatioStretch,
                                         topmost: false,
                                     })
            .unwrap();
        window.limit_update_rate(Some(Duration::from_millis(20)));
        let mut menu = Menu::new("Control").unwrap();
        menu.add_item("Load Scene", LOAD_SCENE_BTN).shortcut(Key::N, MENU_KEY_CTRL).build();
        menu.add_separator();
        menu.add_item("Start Rendering", START_BTN).shortcut(Key::Enter, MENU_KEY_CTRL).build();
        menu.add_item("Pause Rendering", PAUSE_BTN).shortcut(Key::Period, MENU_KEY_CTRL).build();
        menu.add_separator();
        menu.add_item("Save", SAVE_BTN).shortcut(Key::S, MENU_KEY_CTRL).build();
        window.add_menu(&menu);
        window.add_menu(&Menu::new("Help").unwrap());
        unimplemented!()
        // Self {
        //     film,
        //     buffer,
        //     window,
        // }
    }

    pub fn run(&mut self) {
        unimplemented!()
        // while self.window.is_open() {
        //     self.window.update_with_buffer(&self.buffer, self.film.width() as usize, self.film.height() as usize).unwrap();
        //     self.window.is_menu_pressed().map(|menu_id| {
        //         match menu_id {
        //             LOAD_SCENE_BTN => unimplemented!(),
        //             START_BTN => {
        //                 println!("Start clicked!");
        //             }
        //             PAUSE_BTN => unimplemented!(),
        //             SAVE_BTN => unimplemented!(),
        //             _ => unreachable!()
        //         }
        //     });
        // }
    }
}

fn setup_scene() -> Scene<Sphere, bsdf::Simple, texture::Uniform> {
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

fn setup_camera() -> Camera<camera::Perspective> {
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
