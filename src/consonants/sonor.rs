use crate::generators::AdditiveGenerator;

pub struct SonorGeneratorCollection {
    pub r: AdditiveGenerator,
    pub l: AdditiveGenerator,
}

impl SonorGeneratorCollection {
    pub fn new() -> Self {
        Self {
            r: AdditiveGenerator::new(310.0, 1060.0, 1380.0),
            l: AdditiveGenerator::new(310.0, 1050.0, 2280.0),
        }
    }
}
