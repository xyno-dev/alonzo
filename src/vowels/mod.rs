use crate::generators::AdditiveGenerator;

pub struct VowelGeneratorCollection {
    pub ah: AdditiveGenerator,
    pub ee: AdditiveGenerator,
    pub ih: AdditiveGenerator,
    pub oo: AdditiveGenerator,
    pub e: AdditiveGenerator,
    pub o: AdditiveGenerator,
    pub a: AdditiveGenerator,
    pub uh: AdditiveGenerator,
}

impl VowelGeneratorCollection {
    pub fn new() -> Self {
        Self {
            ah: AdditiveGenerator::new(700.0, 1220.0, 2600.0),
            ee: AdditiveGenerator::new(270.0, 2300.0, 3000.0),
            ih: AdditiveGenerator::new(400.0, 1800.0, 2570.0),
            oo: AdditiveGenerator::new(320.0, 900.0, 2200.0),
            e: AdditiveGenerator::new(330.0, 2020.0, 2300.0),
            o: AdditiveGenerator::new(540.0, 1100.0, 2300.0),
            a: AdditiveGenerator::new(620.0, 1660.0, 2430.0),
            uh: AdditiveGenerator::new(620.0, 1200.0, 2150.0),
        }
    }
}
