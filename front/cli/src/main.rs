use chippy::emu::{
    input::Key,
    vm::{ProgramState, Vm},
};
use crossterm::event::KeyCode;
use eyre::{Result, WrapErr};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
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

    let opts = Opt::from_args();

    let bytes = std::fs::read(&opts.filepath).wrap_err("Failed to open c8 file")?;
    let mut vm = Vm::new();
    vm.load(bytes);

    // Because the parent thread that is spawning this thread is the main one we dont have to join
    // it at the end of the program. As it is the end of the program it will be terminated.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || loop {
        let event = crossterm::event::read().expect("failed to read crossterm event");
        tx.send(event).expect("failed to send event");
    });

    crossterm::terminal::enable_raw_mode().unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let ctrlc_running_handle = running.clone();
    ctrlc::set_handler(move || {
        ctrlc_running_handle.store(false, Ordering::SeqCst);
    })?;

    let frame = Duration::from_millis((1000 / opts.fps) as u64);
    while running.load(Ordering::SeqCst) {
        let now = Instant::now();

        vm.input.clear();
        while let Ok(event) = rx.try_recv() {
            match event {
                crossterm::event::Event::Key(key) => match key.code {
                    KeyCode::Esc => running.store(false, Ordering::SeqCst),
                    KeyCode::Char('q') => running.store(false, Ordering::SeqCst),
                    KeyCode::Char('0') => vm.input.key_down(Key::Zero),
                    KeyCode::Char('1') => vm.input.key_down(Key::One),
                    KeyCode::Char('2') => vm.input.key_down(Key::Two),
                    KeyCode::Char('3') => vm.input.key_down(Key::Three),
                    KeyCode::Char('4') => vm.input.key_down(Key::Four),
                    KeyCode::Char('5') => vm.input.key_down(Key::Five),
                    KeyCode::Char('6') => vm.input.key_down(Key::Six),
                    KeyCode::Char('7') => vm.input.key_down(Key::Seven),
                    KeyCode::Char('8') => vm.input.key_down(Key::Eight),
                    KeyCode::Char('9') => vm.input.key_down(Key::Nine),
                    KeyCode::Char('a') => vm.input.key_down(Key::A),
                    KeyCode::Char('b') => vm.input.key_down(Key::B),
                    KeyCode::Char('c') => vm.input.key_down(Key::C),
                    KeyCode::Char('d') => vm.input.key_down(Key::D),
                    KeyCode::Char('e') => vm.input.key_down(Key::E),
                    KeyCode::Char('f') => vm.input.key_down(Key::F),
                    KeyCode::Char(_) => {}
                    _ => {}
                },
                _ => {}
            }
        }

        match vm.cycle() {
            ProgramState::Continue => {}
            ProgramState::Stop => running.store(false, Ordering::SeqCst),
        }

        vm.decrement_registers();
        if vm.should_draw {
            vm.should_draw = false;
            // TODO: render
        }

        if let Some(remaining) = frame.checked_sub(now.elapsed()) {
            std::thread::sleep(remaining);
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();

    Ok(())
}
