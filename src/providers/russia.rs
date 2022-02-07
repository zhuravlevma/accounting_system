use super::{Product, ProductCreator, Responsible};

pub struct RussianGas;
impl Product for RussianGas {
    fn get_price(&self) -> f64 {
        12.0
    }
}
pub struct RussiaResponsible;
impl Responsible for RussiaResponsible {
    fn get_info(&self) -> String {
        "We're russia, nuclear country".to_string()
    }
}
pub struct Gasprom;

impl ProductCreator for Gasprom {
    type Product = RussianGas;
    type Responsible = RussiaResponsible;

    fn create_product(&self) -> Self::Product {
        RussianGas
    }

    fn create_responsible(&self) -> Self::Responsible {
        RussiaResponsible
    }
}

pub fn russia_factory() -> impl ProductCreator {
    Gasprom
}
