use crate::generators::ExtendedAdditiveGenerator;

pub struct NasalGeneratorCollection {
    pub m: ExtendedAdditiveGenerator,
}

impl NasalGeneratorCollection {
    pub fn new() -> Self {
        Self {
            m: ExtendedAdditiveGenerator::new(270.0, 450.0, 0.05, 0.05, 480.0, 1270.0, 2130.0),
        }
    }
}
