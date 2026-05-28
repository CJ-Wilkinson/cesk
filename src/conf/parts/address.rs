use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone)]
pub struct Address {
    a: usize,
}

impl Address {
    pub fn get_address(&mut self) -> Address {
        let addr = self.a;
        self.a += 1;
        Address { a: addr }
    }
    pub fn new(addr: usize) -> Self {
        Self { a: addr }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.a)
    }
}
