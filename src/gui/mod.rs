use crate::*;
use minifb::*;
use std::time::{Duration, Instant};
use std::sync::{Arc, Condvar, Mutex};
use crate::utils::{ToHexColor, ToImageBuffer, GammaCorrection};
use std::thread::spawn;

mod config;
mod setup;
mod kernel;
mod event_loop;

use config::*;
use setup::*;
use kernel::*;
use event_loop::*;

#[derive(Debug, Copy, Clone)]
enum KernelCommand {
    Start,
    /// To re-render
    Restart,
    /// To finish normally
    Finish,
    None,
}

type UnsafeContext<G, B, T, C, S> = Arc<UnsafeWrapper<Context<G, B, T, C, S>>>;
type CommandSender = Arc<(Condvar, Mutex<KernelCommand>)>;


pub fn gui() {
    let window = setup_window();
    let context = Arc::new(UnsafeWrapper::new(Context {
        scene: setup_scene(),
        camera: setup_camera(),
        sampler: sampler::Independent,
        film: Film::new(WIDTH, HEIGHT),
        progress: Float::nan(),
        terminate_request: false,
    }));
    let integrator = setup_integrator();
    let (kernel_join, kernel_command) = setup_kernel(context.clone(), integrator);
    // window.set_title(&format!("{} (press `Command + Enter` to start)", PHAROSA));
    kernel_command.send(KernelCommand::Start);
    event_loop(window, context, kernel_command);
    kernel_join.join().unwrap();
}

trait SendCommand {
    fn send(&self, cmd: KernelCommand);
}

impl SendCommand for CommandSender {
    fn send(&self, cmd: KernelCommand) {
        let (has_cmd, cmd_type) = &**self;
        *cmd_type.lock().unwrap() = cmd;
        has_cmd.notify_one();
    }
}
