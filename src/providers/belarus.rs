use super::{Product, ProductCreator, Responsible};

pub struct BelarusMilk;
impl Product for BelarusMilk {
    fn get_price(&self) -> f64 {
        34.5
    }
}
pub struct BelarusResponsible;
impl Responsible for BelarusResponsible {
    fn get_info(&self) -> String {
        "Tractor cures coronavirus!".to_string()
    }
}
pub struct BelarusMilkFactory;
impl ProductCreator for BelarusMilkFactory {
    type Product = BelarusMilk;
    type Responsible = BelarusResponsible;

    fn create_product(&self) -> Self::Product {
        BelarusMilk
    }

    fn create_responsible(&self) -> Self::Responsible {
        BelarusResponsible
    }
}

pub fn belarus_factory() -> impl ProductCreator {
    BelarusMilkFactory
}
