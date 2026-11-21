use crate::generators::{AdditiveGenerator, NoiseGenerator};

pub struct FricativeGeneratorCollection {
    pub f: AdditiveGenerator,
    pub v: AdditiveGenerator,
    pub s: NoiseGenerator,
}

impl FricativeGeneratorCollection {
    pub fn new() -> Self {
        Self {
            f: AdditiveGenerator::new(340.0, 1100.0, 2080.0),
            v: AdditiveGenerator::new(220.0, 1100.0, 2080.0),
            s: NoiseGenerator::new(),
        }
    }
}
