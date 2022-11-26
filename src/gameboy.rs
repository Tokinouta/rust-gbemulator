use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::Cpu,
    gpu::Gpu,
    mbc::CartridgeHeader,
    memory::{self, Memory},
};

struct GameBoy {
    cpu: Cpu,
    memory: Rc<RefCell<Memory>>,
    gpu: Rc<RefCell<Gpu>>,
}

impl GameBoy {
    pub fn new(header: CartridgeHeader) -> Self {
        let gpu = Rc::new(RefCell::new(Gpu::new()));
        let memory = Rc::new(RefCell::new(Memory::new(header, gpu.clone())));
        let cpu = Cpu::new(memory.clone());
        Self { cpu, memory, gpu }
    }
}
