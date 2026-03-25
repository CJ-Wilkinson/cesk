use super::conf::Address;
use crate::ast::*;
use crate::visit::SuccessorVisitor;

use crate::visit::*;

use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

type SuccessorMap = HashMap<Rc<Stmt>, Rc<Stmt>>;

pub struct ProgramHandler {
    pub addr: Address,
    pub successor_map: SuccessorMap,
    pub program: Program,
}

impl From<(SuccessorMap, Program)> for ProgramHandler {
    fn from(item: (SuccessorMap, Program)) -> Self {
        Self {
            addr: Address::new(),
            successor_map: item.0,
            program: item.1,
        }
    }
}

impl From<Program> for ProgramHandler {
    fn from(prog: Program) -> Self {
        Self {
            addr: Address::new(),
            successor_map: {
                let mut sv = SuccessorVisitor::new();
                prog.traverse(&mut sv);
                let SuccessorVisitor { map } = sv;
                map
            },
            program: prog,
        }
    }
}

impl ProgramHandler {
    pub fn new() -> Self {
        /*
        Creates a completely empty ProgramHandler
        */
        Self {
            addr: Address::new(),
            successor_map: SuccessorMap::new(),
            program: Program::new(),
        }
    }
    pub fn get_address(&mut self) -> Address {
        /*
        Wrapper around the get_address function
        */
        self.addr.get_address()
    }
    pub fn successor_lookup(&self, key: Rc<Stmt>) -> Rc<Stmt> {
        /*
        Get the successor of the given key
        */
        match self.successor_map.get(&key) {
            Some(stmt) => stmt.clone(),
            None => Rc::new(Stmt::Break), // TODO: This will be the case that there is not successor
        }
    }
    pub fn function_lookup(&self, _fun_id: &Name) -> Option<(ParamList, Arguments)> {
        /*
        Get the function given
        */
        todo!()
    }
}
