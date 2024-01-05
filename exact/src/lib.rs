use fraction::GenericFraction;
// use std::collections::{HashSet, hash_map::RandomState};

pub mod expr_rc;
pub mod structs;
pub mod io;
pub use num::{One, Zero};

pub type F = GenericFraction<u32>;

// pub use expr_rc::*;
// pub struct Ring(HashSet<ExprLayer, RandomState>);

// pub enum ExprLayer<A>

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
