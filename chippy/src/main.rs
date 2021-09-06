use chippy::emu::vm::Vm;

fn main() {
    let bytes = std::fs::read("roms/pong.ch8").unwrap();
    let mut vm = Vm::new();
    vm.load(bytes);

    for _ in 0..10000 {
        vm.cycle();
    }

    println!("{}", vm.gpu);
}
