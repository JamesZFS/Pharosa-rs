use crate::*;
use minifb::*;
use std::time::{Duration, Instant};
use std::sync::{Arc, Condvar, Mutex};
use crate::utils::{ToHexColor, ToImageBuffer, GammaCorrection};
use std::thread::spawn;

mod config;
mod setup;

use config::*;
use setup::*;

pub fn gui() {
    let window = setup_window();
    let context = Arc::new(UnsafeWrapper::new(Context {
        scene: setup_scene_cornell(),
        camera: setup_camera_cornell(),
        sampler: sampler::Independent,
        film: Film::new(WIDTH, HEIGHT),
        progress: 0.,
        terminate_request: false,
    }));
    let integrator = setup_integrator();
    event_loop(window, context, integrator);
}

#[derive(Debug, Copy, Clone)]
enum KernelCommand {
    /// To re-render
    Restart,
    /// To finish normally
    Finish,
    None,
}

fn event_loop(mut window: Window,
              context: Arc<UnsafeWrapper<Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>>>,
              integrator: impl Integrator) {
    // unsafe get handlers
    let Context { ref film, camera, ref progress, terminate_request, .. } = unsafe { context.get_mut() };
    // buffer for the window to display
    let mut buffer = Vec::with_capacity(film.size());
    let kernel_command = Arc::new((Condvar::new(), Mutex::new(KernelCommand::None)));

    let kernel;
    { // spawn the kernel thread
        let context = context.clone();
        let kernel_command = kernel_command.clone();
        // do rendering in 'kernel' thread
        kernel = spawn(move || {
            println!("Kernel started.");
            let context = unsafe { context.get_mut() };
            #[cfg(debug_assertions)] {
                println!("{:#?}", context);
                println!("{:#?}", integrator);
            }
            let tic = Instant::now();
            loop { // render loop
                context.terminate_request = false;
                integrator.render(context);
                // wait for a gui command:
                let (has_cmd, cmd_type) = &*kernel_command;
                let mut cmd_type = cmd_type.lock().unwrap();
                while let KernelCommand::None = &*cmd_type { // wait while no commands
                    cmd_type = has_cmd.wait(cmd_type).unwrap();
                }
                match *cmd_type {
                    KernelCommand::Restart => *cmd_type = KernelCommand::None, // go back to render loop
                    KernelCommand::Finish => {
                        *cmd_type = KernelCommand::None;
                        break;
                    }
                    _ => unreachable!()
                }
            }
            println!("Kernel finished in {:?}", tic.elapsed());
        });
    }

    while window.is_open() {
        let progress = *progress;
        if progress == 1. {
            window.set_title(&format!("{} (done)", PHAROSA));
        } else {
            window.set_title(&format!("{} (rendering {:.1}%)", PHAROSA, progress * 100.));
        }

        // refresh display
        update_buffer(&mut buffer, film);
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();

        // process menu events
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                LOAD_SCENE_BTN => unimplemented!(),
                START_BTN => {
                    println!("Start clicked!");
                }
                PAUSE_BTN => unimplemented!(),
                SAVE_BTN => save(film),
                _ => unreachable!()
            }
        });

        // process keyboard events
        window.get_keys().map(|keys| {
            for key in keys {
                if match key {
                    Key::W => { // move front, notice camera's z-axis points towards the user
                        camera.translate(vec3(0., 0., -CAMERA_TRANSLATION_STEP));
                        true
                    }
                    Key::S => { // move back
                        camera.translate(vec3(0., 0., CAMERA_TRANSLATION_STEP));
                        true
                    }
                    Key::A => { // move left, notice camera's x-axis points towards screen's right
                        camera.translate(vec3(-CAMERA_TRANSLATION_STEP, 0., 0.));
                        true
                    }
                    Key::D => { // move right
                        camera.translate(vec3(CAMERA_TRANSLATION_STEP, 0., 0.));
                        true
                    }
                    Key::Space => { // move up
                        camera.translate(vec3(0., CAMERA_TRANSLATION_STEP, 0.));
                        true
                    }
                    Key::LeftShift | Key::RightShift => { // move down
                        camera.translate(vec3(0., -CAMERA_TRANSLATION_STEP, 0.));
                        true
                    }
                    Key::Left => { // rotate along y-axis
                        camera.rotate(Vector3::unit_y(), CAMERA_ROTATION_STEP);
                        true
                    }
                    Key::Right => { // rotate along y-axis
                        camera.rotate(Vector3::unit_y(), -CAMERA_ROTATION_STEP);
                        true
                    }
                    Key::Up => { // rotate along x-axis
                        camera.rotate(Vector3::unit_x(), CAMERA_ROTATION_STEP);
                        true
                    }
                    Key::Down => { // rotate along x-axis
                        camera.rotate(Vector3::unit_x(), -CAMERA_ROTATION_STEP);
                        true
                    }
                    Key::Q => { // rotate along z-axis
                        camera.rotate(Vector3::unit_z(), CAMERA_ROTATION_STEP);
                        true
                    }
                    Key::E => { // rotate along x-axis
                        camera.rotate(Vector3::unit_z(), -CAMERA_ROTATION_STEP);
                        true
                    }
                    _ => false,
                }
                { // notify kernel to advance
                    *terminate_request = true;
                    let (has_cmd, cmd_type) = &*kernel_command;
                    *cmd_type.lock().unwrap() = KernelCommand::Restart;
                    has_cmd.notify_one();
                }
            }
        });
    }
    println!("Waiting for kernel to finish...");
    let (has_cmd, cmd_type) = &*kernel_command;
    *cmd_type.lock().unwrap() = KernelCommand::Finish;
    has_cmd.notify_one();
    kernel.join().unwrap();
}

fn update_buffer(buffer: &mut Vec<u32>, film: &Film) {
    buffer.clear();
    buffer.extend(film.to_raw().iter().map(|s| s.gamma_correction().to_hex_color()));
}

fn save(film: &Film) {
    film.to_image_buffer().save(SAVE_PATH).unwrap();
    println!("Result saved to '{}'", SAVE_PATH);
}

