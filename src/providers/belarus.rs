use super::{Product, ProductCreator, Provider};

pub struct BelarusMilk;
impl Product for BelarusMilk {
    fn get_price(&self) -> f64 {
        34.5
    }
}
pub struct BelarusProvider;
impl Provider for BelarusProvider {
    fn get_info(&self) -> String {
        "Tractor cures coronavirus!".to_string()
    }
}
pub struct BelarusMilkFactory;
impl ProductCreator for BelarusMilkFactory {
    type Product = BelarusMilk;
    type Provider = BelarusProvider;

    fn create_product(&self) -> Self::Product {
        BelarusMilk
    }

    fn create_provider(&self) -> Self::Provider {
        BelarusProvider
    }
}

pub fn belarus_factory() -> impl ProductCreator {
    BelarusMilkFactory
}
