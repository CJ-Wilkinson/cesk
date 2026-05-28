use super::address::Address;
use crate::ast::*;
use std::collections::HashMap;
use std::rc::Rc;
// use std::fmt::{Display, Formatter, Error};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Store(HashMap<Address, Rc<Value>>);
pub type Store = HashMap<Address, Rc<Value>>;

// impl Store {
//     pub fn new() -> Self {
//         Self(HashMap::new())
//     }
//     pub fn insert(&mut self, addr: Address, val: Rc<Value>) {
//         self.0.insert(addr, val);
//     }
//     pub fn get(&self, addr: &Address) -> Option<Rc<Value>> {
//         match self.0.get(addr) {
//             Some(val) => Some(val.clone()),
//             None => None,
//         }
//     }
// }
//
// impl Display for Store {
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
