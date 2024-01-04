use super::*;

use num::{One, Zero};

impl One for Expr {
  fn one() -> Self {
      Expr::from(1u32)
  }

  fn is_one(&self) -> bool
    where
        Self: PartialEq, {
    *self == Expr::from(1u32)
  }

  fn set_one(&mut self) {
    todo!()
  }
}

impl Zero for Expr {
  fn zero() -> Self {
      Expr::from(0u32)
  }

  fn is_zero(&self) -> bool {
      *self == Expr::from(0u32)
  }

  fn set_zero(&mut self) {

    // let e = &mut Expr::zero();
    // self = e;
  }
}

/// If we can convert to fraction, it's a numeral
impl From<u32> for Expr{
  fn from(value: u32) -> Self {
    Expr::from(F::from(value))
  }
}

impl From<F> for Expr {
  fn from(value: F) -> Self {
    Expr::Val(Rc::new(value))
  }
}

/// from const tuple
impl From<(char, f64)> for Expr {
  fn from(value: (char, f64)) -> Self {
    let (c, f) = value;
    Expr::Const(Rc::new(Const { ch: c, f64: f }))
  }
}

/// From a sum-type vector: Vec<(F, Expr)>
impl From<Vec<(F, Expr)>> for Expr {
  fn from(value: Vec<(F, Expr)>) -> Self {
    if value.len() == 0 {return Expr::zero();}
    if value.len() == 1 && value[0].0 == F::one() {return value[0].1.clone();}
    Expr::Sum(Rc::new(Sum { terms: value }))
  }
}

/// From a prod-type vector: Vec<(Expr, F)>
impl From<Vec<(Expr, F)>> for Expr {
  fn from(value: Vec<(Expr, F)>) -> Self {
    if value.len() == 0 {return Expr::zero();}
    if value.len() == 1 && value[0].1 == F::one() {return value[0].0.clone();}
    Expr::Prod(Rc::new(Prod { factors: value }))
  }
}

#[cfg(test)]
mod test_from {
  use super::*;

  #[test]
  fn test_from() {
    assert_eq!(Expr::from(('π',std::f64::consts::PI)), Expr::c_pi());
    let half = F::new(1u8,2u8); // <- for convenience
    let val_1 = Expr::from(1u32);
    let val_5 = Expr::from(5u32);
    let sqrt_5 = Expr::sqrt_expr(val_5);
    let phi = Expr::from(
      vec![(half,val_1.clone()),(half,sqrt_5.clone())]
    );
    assert_eq!(phi, Expr::sum_phi());
    let phi_sq = Expr::from(
      vec![(F::new(3u8,2u8),val_1.clone()),(half,sqrt_5.clone())]
    );
  }

  #[test]
  fn test_from_shared_rc() {

  }
}