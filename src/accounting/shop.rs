use crate::Unit;

pub struct Shop {
    units: Vec<Box<dyn Unit>>,
}

impl Unit for Shop {
    fn calculate_price(&self) -> f64 {
        self.units.iter().map(|elem| elem.calculate_price()).sum()
    }
}

impl Shop {
    pub fn add(&mut self, unit: Box<dyn Unit>) {
        self.units.push(unit)
    }
}

impl Default for Shop {
    fn default() -> Self {
        Self { units: vec![] }
    }
}
