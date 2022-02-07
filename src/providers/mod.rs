pub use belarus::{belarus_factory, BelarusMilk, BelarusMilkFactory, BelarusResponsible};
pub use russia::{russia_factory, Gasprom, RussiaResponsible, RussianGas};

pub trait Product {
    fn get_price(&self) -> f64;
}

pub trait Responsible {
    fn get_info(&self) -> String;
}

pub trait ProductCreator {
    type Product: Product;
    type Responsible: Responsible;

    fn create_product(&self) -> Self::Product;
    fn create_responsible(&self) -> Self::Responsible;
}

pub mod belarus;
pub mod russia;
