pub trait Unit {
    fn calculate_price(&self) -> f64;
}

pub struct Order {
    price_purchase: f64,
    markup_percentage: u32,
}

impl Unit for Order {
    fn calculate_price(&self) -> f64 {
        self.price_purchase * (100 + self.markup_percentage) as f64
    }
}

impl Order {
    pub fn new(price_purchase: f64, markup_percentage: u32) -> Self {
        Self {
            price_purchase,
            markup_percentage,
        }
    }
}

struct Warehouse {
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
    pub fn new() -> Self {
        Self { units: vec![] }
    }
    pub fn add(&mut self, unit: Box<dyn Unit>) {
        self.units.push(unit);
    }
}

struct Shop {
    units: Vec<Box<dyn Unit>>,
}

impl Unit for Shop {
    fn calculate_price(&self) -> f64 {
        let mut total: f64 = 0.0;
        for elem in &self.units {
            total += elem.calculate_price();
        }
        total
    }
}

impl Shop {
    pub fn new() -> Self {
        Self { units: vec![] }
    }
}
