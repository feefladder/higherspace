use fraction::GenericFraction;
// use std::collections::{HashSet, hash_map::RandomState};

pub mod expr_rc;
pub mod expr_field;
pub mod io_traits;

pub use num::{One, Zero};

pub type FType = u32;
pub type F = GenericFraction<FType>;

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
