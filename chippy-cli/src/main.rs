use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use chippy::emu::vm::{ProgramState, Vm};
use eyre::{Result, WrapErr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "chippy")]
struct Opt {
    /// Set fps
    #[structopt(short, long, default_value = "60")]
    fps: usize,

    #[structopt(name = "FILE")]
    filepath: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::from_args();
    let bytes = std::fs::read(&opt.filepath).wrap_err("Failed to open c8 file")?;

    let mut vm = Vm::new();
    vm.load(bytes);

    let running = Arc::new(AtomicBool::new(true));
    let ctrlc_running_handle = running.clone();

    ctrlc::set_handler(move || {
        ctrlc_running_handle.store(false, Ordering::SeqCst);
    })?;

    let frame = Duration::from_millis((1000 / opt.fps) as u64);
    // let mut last_update = Instant::now();
    while running.load(Ordering::SeqCst) {
        let now = Instant::now();

        match vm.cycle() {
            ProgramState::Continue => {}
            ProgramState::Stop => running.store(false, Ordering::SeqCst),
        }

        // let time_difference = now.checked_duration_since(last_update);
        // if let Some(elasped) = time_difference {
        //     if elasped > Duration::from_millis(10) {
        //         last_update = now;
        //         if vm.should_draw {
        //             vm.should_draw = false;
        //             // TODO: render
        //         }
        //     }
        // }

        vm.decrement_registers();
        if vm.should_draw {
            vm.should_draw = false;
            // TODO: render
        }

        if let Some(remaining) = frame.checked_sub(now.elapsed()) {
            std::thread::sleep(remaining);
        }
    }

    Ok(())
}
