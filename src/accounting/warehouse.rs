use crate::accounting::product::Product as DomainProduct;
use crate::providers::transportation::{
    BelarusTransportation, DynTransportation, RussianTransportation, Transportation,
};
use crate::providers::{BelarusMilkFactory, Gasprom, Product};
use crate::Unit;
pub struct Warehouse {
    units: Vec<Box<dyn Unit>>,
}

impl Unit for Warehouse {
    fn calculate_price(&self) -> f64 {
        self.units.iter().map(|elem| elem.calculate_price()).sum()
    }
}

impl Warehouse {
    pub fn add(&mut self, unit: Box<dyn Unit>) {
        self.units.push(unit);
    }
    pub fn buy_gas(&mut self, transportation: RussianTransportation, factory: Gasprom) {
        let product = transportation.transport(factory);
        let unit = DomainProduct::new(product.get_price(), 20.0);
        self.units.push(Box::new(unit));
    }
    pub fn buy_milk(&mut self, transportation: BelarusTransportation, factory: BelarusMilkFactory) {
        let product = transportation.transport(factory);
        let unit = DomainProduct::new(product.get_price(), 20.0);
        self.units.push(Box::new(unit));
    }
    pub fn buy_product_from_stock(
        &mut self,
        transportation: Box<dyn DynTransportation>,
        product: Box<dyn Product>,
    ) {
        let product = transportation.transport(product);
        let unit = DomainProduct::new(product.get_price(), 20.0);
        self.units.push(Box::new(unit));
    }
}

impl Default for Warehouse {
    fn default() -> Self {
        Self { units: vec![] }
    }
}
