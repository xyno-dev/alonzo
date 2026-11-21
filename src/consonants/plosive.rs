use crate::generators::PlosiveGenerator;

pub struct PlosiveGeneratorCollection {
    pub d: PlosiveGenerator,
    pub g: PlosiveGenerator,
    pub k: PlosiveGenerator,
    pub t: PlosiveGenerator,
}

impl PlosiveGeneratorCollection {
    pub fn new() -> Self {
        Self {
            d: PlosiveGenerator::new(200.0, 1600.0, 2600.0, 1.0, 0.1, 0.5),
            g: PlosiveGenerator::new(200.0, 1990.0, 2850.0, 1.0, 0.1, 0.5),
            k: PlosiveGenerator::new(300.0, 1700.0, 2850.0, 0.7, 1.0, 0.3),
            t: PlosiveGenerator::new(400.0, 1600.0, 2600.0, 0.7, 1.0, 0.3),
        }
    }
}
