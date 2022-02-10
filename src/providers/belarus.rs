use super::{Product, ProductCreator, Responsible};
use crate::providers::Command;

pub struct BelarusMilk;
impl Product for BelarusMilk {
    fn get_price(&self) -> f64 {
        34.5
    }
}
pub struct BelarusResponsible {
    pub name: String,
}
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

    fn create_responsible(&self, name: &str) -> Self::Responsible {
        BelarusResponsible {
            name: String::from(name),
        }
    }
}

#[derive(Default)]
pub struct KGB {
    commands: Vec<Box<dyn Command>>,
}
impl KGB {
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self) {
        self.commands.iter().for_each(|elem| {
            println!("{}", elem.execute());
        })
    }
}

pub struct FileCriminalCase;
impl Command for FileCriminalCase {
    fn execute(&self) -> &str {
        "To initiate criminal proceedings!"
    }
}
pub struct DismissFromOffice;
impl Command for DismissFromOffice {
    fn execute(&self) -> &str {
        "Dismiss from office!"
    }
}
pub struct RaiseEnterprise;
impl Command for RaiseEnterprise {
    fn execute(&self) -> &str {
        "Raise me enterprise!"
    }
}

pub fn belarus_factory() -> impl ProductCreator {
    BelarusMilkFactory
}
