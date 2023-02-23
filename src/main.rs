
#[derive(Debug, Default)]
struct Device6502 {
    a: u8, /* accumulator */
    x: u8, /* x index register */
    y: u8, /* y index register */
    sp: u8, /* stack pointer  */
    pc: u8, /* program counter */
    p: u8, /* processor flags */
}

impl Device6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            p: 0,
            
        }
    }

    pub fn tick(&mut self) {

    }


}

fn main() {

    println!("Started");

    let mut device = Device6502::new();

    for _ in 1..=100 {
        device.tick();
    }


    println!("Finished");
}
