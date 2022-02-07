pub use belarus::{belarus_factory, BelarusMilk, BelarusMilkFactory, BelarusProvider};
pub use russia::{russia_factory, Gasprom, RussiaProvider, RussianGas};

pub trait Product {
    fn get_price(&self) -> f64;
}

pub trait Provider {
    fn get_info(&self) -> String;
}

pub trait ProductCreator {
    type Product: Product;
    type Provider: Provider;

    fn create_product(&self) -> Self::Product;
    fn create_provider(&self) -> Self::Provider;
}

pub mod belarus;
pub mod russia;
