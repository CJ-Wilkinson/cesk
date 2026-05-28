use super::address::Address;
use crate::ast::*;
use std::collections::HashMap;
// use std::fmt::{Display, Formatter, Error};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Env(pub HashMap<Name, Address>);
pub type Env = HashMap<Name, Address>;

// impl Env {
//     pub fn new() -> Self {
//         Self(HashMap::new())
//     }
//     pub fn insert(&mut self, name: Name, addr: Address) {
//         self.0.insert(name, addr);
//     }
//     pub fn get(&self, name: &Name) -> Option<&Address> {
//         self.0.get(name)
//     }
// }

// impl Display for Env {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//         write!(
//             f,
//             "[{}]",
//             self.0
//                 .iter()
//                 .map(|(key, value)| format!("{} -> {}", key, value))
//                 .collect::<Vec<_>>()
//                 .join(", "),
//         )
//     }
// }
