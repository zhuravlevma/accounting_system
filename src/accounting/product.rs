use crate::Unit;

pub struct Product {
    price_purchase: f64,
    markup_percentage: f64,
}

impl Unit for Product {
    fn calculate_price(&self) -> f64 {
        let total_percentage = 100.0 + self.markup_percentage;
        let share = total_percentage / 100.0;
        self.price_purchase * share
    }
}

impl Product {
    pub fn new(price_purchase: f64, markup_percentage: f64) -> Self {
        Self {
            price_purchase,
            markup_percentage,
        }
    }
}
