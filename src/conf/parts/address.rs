use crate::ast::Value;
use std::fmt::{Display, Error, Formatter};
use std::ops::Add;

pub struct AddressError; // ! this sux even worse, fix this

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone)]
pub struct Address {
    pub a: usize,
}

impl Add<i32> for &Address {
    type Output = Address;

    fn add(self, rhs: i32) -> Self::Output {
        Address {
            a: self.a + rhs as usize,
        }
    }
}

impl Add<&Address> for i32 {
    type Output = Address;

    fn add(self, rhs: &Address) -> Self::Output {
        Address {
            a: rhs.a + self as usize,
        }
    }
}

impl Add<usize> for &Address {
    type Output = Address;

    fn add(self, rhs: usize) -> Self::Output {
        Address { a: rhs + self.a }
    }
}

impl Add<&Address> for usize {
    type Output = Address;

    fn add(self, rhs: &Address) -> Self::Output {
        Address { a: rhs.a + self }
    }
}

impl Address {
    //returns an address that is not being used
    pub fn get_address(&mut self) -> Address {
        let addr = self.a;
        self.a += 1;
        Address { a: addr }
    }

    pub fn address_offset(&self, offset: i32) -> Self {
        Self {
            a: self.a + offset as usize,
        }
    }

    pub fn get_index_info(offset: i32, arr: &Value) -> Result<Address, AddressError> {
        if offset < 0 {
            return Err(AddressError);
        }

        let offset = offset as usize;

        if let Value::ArrayV {
            size,
            start_of_array,
        } = arr
        {
            let potential = offset + start_of_array;
            if start_of_array + *size < potential {
                Err(AddressError)
            } else {
                Ok(potential)
            }
        } else {
            Err(AddressError)
        }
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
