use super::{Product, ProductCreator, Provider};

pub struct RussianGas;
impl Product for RussianGas {
    fn get_price(&self) -> f64 {
        12.0
    }
}
pub struct RussiaProvider;
impl Provider for RussiaProvider {
    fn get_info(&self) -> String {
        "We're russia, nuclear country".to_string()
    }
}
pub struct Gasprom;

impl ProductCreator for Gasprom {
    type Product = RussianGas;
    type Provider = RussiaProvider;

    fn create_product(&self) -> Self::Product {
        RussianGas
    }

    fn create_provider(&self) -> Self::Provider {
        RussiaProvider
    }
}

pub fn russia_factory() -> impl ProductCreator {
    Gasprom
}
