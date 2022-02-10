use super::{Product, ProductCreator, Responsible};
use crate::providers::Command;

pub struct RussianGas;
impl Product for RussianGas {
    fn get_price(&self) -> f64 {
        12.0
    }
}
pub struct RussiaResponsible {
    pub name: String,
}
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

    fn create_responsible(&self, name: &str) -> Self::Responsible {
        RussiaResponsible {
            name: String::from(name),
        }
    }
}

#[derive(Default)]
pub struct Miller {
    commands: Vec<Box<dyn Command>>,
}
impl Miller {
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self) {
        self.commands.iter().for_each(|elem| {
            println!("{}", elem.execute());
        })
    }
}

pub struct StealSomeMoney;
impl Command for StealSomeMoney {
    fn execute(&self) -> &str {
        "Steal Money"
    }
}

pub struct StealALotOfMoney;
impl Command for StealALotOfMoney {
    fn execute(&self) -> &str {
        "Steal a lot of Money"
    }
}

pub struct GiveCreditToBelarus;
impl Command for GiveCreditToBelarus {
    fn execute(&self) -> &str {
        "Give credit to Belarus!"
    }
}

pub fn russia_factory() -> impl ProductCreator {
    Gasprom
}
