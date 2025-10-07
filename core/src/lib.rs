use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    steps: u64,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        Emulator { steps: 0 }
    }

    pub fn step(&mut self) -> u64 {
        self.steps += 1;
        self.steps
    }

    pub fn get_steps(&self) -> u64 {
        self.steps
    }
}
