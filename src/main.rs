struct PinID {
    device_id: usize,
    pin_index: usize,
}

#[derive(Default)]
struct Net {
    value: bool,
    connected_io: Vec<PinID>,
}

struct Circuit {
    nets: Vec<Net>,
}

#[derive(Debug)]
struct Device6502 {
    /* Memories and registers, external state */
    io: [bool; 40],
    a: u8, /* accumulator */
    x: u8, /* x index register */
    y: u8, /* y index register */
    sp: u8, /* stack pointer  */
    pc: u8, /* program counter */
    p: u8, /* processor flags */

    /* Helpers */
    last_clk_pin_state: bool,
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
            io: [false; 40],
            last_clk_pin_state: false,
        }
    }

    pub fn tick(&mut self) {
        /* Update state on positive clock edge */

        if (self.io[37] != self.last_clk_pin_state) {
            /* Clock tick here */
        }

        self.last_clk_pin_state = self.io[37];
    }

    pub fn set_io(&mut self, index: usize, value: bool) {
        self.io[index] = value;
    }


}

struct Device28C256;

impl Device28C256 {
    pub fn new() -> Self {
        Self
    }


}

fn main() {

    println!("Started");

    let mut net1: Net = Net::default();

    let mut device = Device6502::new();

    for _ in 1..=100 {
        device.tick();
    }


    println!("Finished");
}
