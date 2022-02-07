use crate::Unit;

pub struct Warehouse {
    units: Vec<Box<dyn Unit>>,
}

impl Unit for Warehouse {
    fn calculate_price(&self) -> f64 {
        let mut total: f64 = 0.0;
        for elem in &self.units {
            total += elem.calculate_price();
        }
        total
    }
}

impl Warehouse {
    pub fn add(&mut self, unit: Box<dyn Unit>) {
        self.units.push(unit);
    }
}

impl Default for Warehouse {
    fn default() -> Self {
        Self { units: vec![] }
    }
}
