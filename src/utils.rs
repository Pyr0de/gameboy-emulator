use std::{
    marker::PhantomData,
    ops::{BitAnd, BitXor},
};

#[derive(Debug)]
pub struct BitFlag<T, U> {
    pub value: T,
    _phantom: PhantomData<U>,
}

impl<T, U> Default for BitFlag<T, U>
where
    T: Default,
    U: Into<T>,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T, U> BitFlag<T, U>
where
    T: Default + Copy + PartialEq + BitAnd<Output = T> + BitXor<Output = T>,
    U: Into<T> + Copy,
{
    pub fn get(&self, flag: U) -> bool {
        self.get_into(flag.into())
    }

    pub fn get_into(&self, flag: T) -> bool {
        self.value & flag != T::default()
    }

    pub fn set(&mut self, flag: U, set_bit: bool) {
        self.set_into(flag.into(), set_bit);
    }

    pub fn set_into(&mut self, flag: T, set_bit: bool) {
        if self.get_into(flag) != set_bit {
            self.value = self.value ^ flag;
        }
    }
}
