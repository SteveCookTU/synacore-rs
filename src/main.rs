use synacore_rs::VirtualMachine;

const BINARY: &[u8] = include_bytes!("../challenge.bin");

fn main() -> Result<(), &'static str> {
    VirtualMachine::default().execute(BINARY)
}
