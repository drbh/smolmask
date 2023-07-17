extern crate num;
extern crate num_traits;

use num::{PrimInt, Unsigned};
use num_traits::FromPrimitive;
use std::marker::PhantomData;

pub struct BoolArray<T>
where
    T: PrimInt + Unsigned + FromPrimitive,
{
    phantom: PhantomData<T>,
}

impl<T> BoolArray<T>
where
    T: PrimInt + Unsigned + FromPrimitive,
{
    fn max_size() -> usize {
        std::mem::size_of::<T>() * 4
    }

    pub fn store(bools: &[bool]) -> Result<T, &'static str> {
        let size_shift = Self::max_size() / 2;
        let size = bools.len();
        if size > size_shift {
            return Err("Input length exceeds max size.");
        }

        let mut integer = T::from_usize(size).ok_or("Size conversion failed")? << size_shift;

        for (i, &val) in bools.iter().enumerate() {
            if size_shift - 1 < i {
                return Err("Size shift is less than index, overflow could occur.");
            }
            if val {
                integer = integer | T::one() << (size_shift - 1 - i);
            }
        }

        Ok(integer)
    }

    pub fn retrieve(integer: T) -> Vec<bool> {
        let size_shift = Self::max_size() / 2;
        let size = (integer >> size_shift).to_usize().unwrap();
        let mut bools = vec![false; size];

        for i in 0..size {
            if size_shift - 1 < i {
                panic!("Size shift is less than index, overflow could occur.");
            }
            bools[i] = (integer & (T::one() << (size_shift - 1 - i))) != T::zero();
        }

        bools
    }

    pub fn length(integer: T) -> usize {
        let size_shift = Self::max_size() / 2;
        (integer >> size_shift).to_usize().unwrap()
    }

    pub fn max_bools() -> usize {
        Self::max_size() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bools = vec![true, false, true, true];
        let integer = BoolArray::<u64>::store(&bools).unwrap();
        assert_eq!(BoolArray::<u64>::length(integer), bools.len());
        assert_eq!(BoolArray::<u64>::retrieve(integer), bools);
    }

    #[test]
    fn test_error_on_exceed_max_size() {
        let bools = vec![true; BoolArray::<u64>::max_size() / 2 + 1];
        assert!(BoolArray::<u64>::store(&bools).is_err());
    }
}