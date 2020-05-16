use super::*;
use std::thread::JoinHandle;

/// spawn the kernel thread
pub(super) fn setup_kernel(context: UnsafeContext<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>,
                    integrator: impl Integrator) -> (JoinHandle<impl Send + 'static>, CommandSender) {
    let kernel_command_ret = Arc::new((Condvar::new(), Mutex::new(KernelCommand::None)));
    let kernel_command = kernel_command_ret.clone();
    // do rendering in 'kernel' thread
    let kernel_join = spawn(move || {
        println!("[Kernel] started.");
        let (has_cmd, cmd_type) = &*kernel_command;
        let context = unsafe { context.get_mut() };
        #[cfg(debug_assertions)] {
            println!("{:#?}", context);
            println!("{:#?}", integrator);
        }
        { // wait for Start command
            let mut cmd_type = cmd_type.lock().unwrap();
            while let KernelCommand::None = &*cmd_type { // wait until not None
                cmd_type = has_cmd.wait(cmd_type).unwrap();
            }
            match *cmd_type {
                KernelCommand::Start => {
                    *cmd_type = KernelCommand::None;
                    // start render loop
                }
                _ => unreachable!()
            }
        }
        loop { // render loop
            context.terminate_request = false;
            let tic = Instant::now();
            integrator.render(context); // if terminate_request is set to true, this function call will early stop
            println!("[Kernel] rendering finished in {:?}", tic.elapsed());
            // wait for a gui command:
            let mut cmd_type = cmd_type.lock().unwrap();
            while let KernelCommand::None = &*cmd_type { // wait while no commands
                cmd_type = has_cmd.wait(cmd_type).unwrap();
            }
            match *cmd_type {
                KernelCommand::Restart | KernelCommand::Start => *cmd_type = KernelCommand::None, // go back to render loop
                KernelCommand::Finish => {
                    *cmd_type = KernelCommand::None;
                    break;
                }
                _ => unreachable!()
            }
        }
        println!("[Kernel] shutdown.");
    });
    (kernel_join, kernel_command_ret)
}