use super::*;

pub(super)
fn event_loop(mut window: Window, context: UnsafeContext<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>,
              kernel_command: CommandSender) {
    // unsafe get handlers
    let Context { ref film, camera, ref progress, terminate_request, .. } = unsafe { context.get_mut() };
    // buffer for the window to display
    let mut buffer = Vec::with_capacity(film.size());

    while window.is_open() {
        let progress = *progress;
        if progress == 1. {
            window.set_title(&format!("{} (done)", PHAROSA));
        } else if progress >= 0. {
            window.set_title(&format!("{} (rendering {:.1}%)", PHAROSA, progress * 100.));
        }

        // refresh display
        update_buffer(&mut buffer, film);
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();

        // process menu events
        window.is_menu_pressed().map(|menu_id| {
            match menu_id {
                LOAD_SCENE_BTN => unimplemented!(),
                START_BTN => kernel_command.send(KernelCommand::Start),
                PAUSE_BTN => {
                    *terminate_request = true; // terminate the render call immediately
                    kernel_command.send(KernelCommand::None);
                }
                SAVE_BTN => save(film),
                _ => unreachable!()
            }
        });

        // process keyboard events
        if progress >= 0. {
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
                        window.set_title(&format!("{} (moving...)", PHAROSA));
                        *terminate_request = true; // terminate the render call immediately
                        kernel_command.send(KernelCommand::Restart);
                    }
                }
            });
        }
    }
    println!("Waiting for kernel to finish...");
    *terminate_request = true; // terminate the render call immediately
    kernel_command.send(KernelCommand::Finish);
}

fn update_buffer(buffer: &mut Vec<u32>, film: &Film) {
    buffer.clear();
    buffer.extend(film.to_raw().iter().map(|s| s.gamma_correction().to_hex_color()));
}

fn save(film: &Film) {
    film.to_image_buffer().save(SAVE_PATH).unwrap();
    println!("Result saved to '{}'", SAVE_PATH);
}
