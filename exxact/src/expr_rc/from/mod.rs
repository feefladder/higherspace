use crate::expr_rc::io::ParseDisplay;

use super::*;

mod pvec;
mod svec;
pub use svec::*;
use fraction::{One, Zero};

pub trait FromRaw<T>: Sized {
  /// Converts to this type from the input type.
  #[must_use]
  fn from_raw(value: T) -> Self;
}

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
    todo!()
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
    // Π[(π,a),(π,b)] -> Π(π,a+b)
    if value.len() == 0 {return Expr::zero();}
    if value.len() == 1 {
      if value[0].0 == F::one() {return value[0].1.clone();}
      if value[0].0 == F::zero() {return Expr::zero();}
    }

    let mut v = value.clone();
    v.retain(|(f,_)| f != &F::zero());
    Expr::Sum(Rc::new(Sum { terms: v }))
  }
}

impl TryFrom<&str> for Expr {
  type Error = lyn::Error;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
      Expr::parse_display(value.to_string())
  }
}

impl TryFrom<String> for Expr {
  type Error = lyn::Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Expr::parse_display(value)
  }
}

#[cfg(test)]
mod test_from {
  // use proptest::arbitrary::any;

use super::*;
  fn build_phi() -> Expr {
    let half = F::new(1u8,2u8); // <- for convenience
    let val_1 = Expr::from(1u32);
    let val_5 = Expr::from(5u32);
    let sqrt_5 = Expr::sqrt_expr(val_5);
    Expr::from(
      vec![(half,val_1.clone()),(half,sqrt_5.clone())]
    )
  }

  fn build_phi_phi2() -> (Expr, Expr) {
    let half = F::new(1u8,2u8); // <- for convenience
    let val_1 = Expr::from(1u32);
    let val_5 = Expr::from(5u32);
    let sqrt_5 = Expr::sqrt_expr(val_5);
    (
      Expr::from(
        vec![(half,val_1.clone()),(half,sqrt_5.clone())]
      ),
      Expr::from(
        vec![(F::new(3u8,2u8),val_1.clone()),(half,sqrt_5.clone())]
      )
    )
  }

  #[test]
  fn test_from() {
    assert_eq!(Expr::from(('π',std::f64::consts::PI)), Expr::c_pi());
    // let half = F::new(1u8,2u8); // <- for convenience
    // let val_1 = Expr::from(1u32);
    // let val_5 = Expr::from(5u32);
    // let sqrt_5 = Expr::sqrt_expr(val_5);
    // let phi = Expr::from(
    //   vec![(half,val_1.clone()),(half,sqrt_5.clone())]
    // );
    assert_eq!(build_phi(), Expr::sum_phi());
  }

  #[test]
  fn test_from_prod() {
    let test_vec = vec![
      (vec![
        (Expr::c_pi(), F::from(2)),
      ],Expr::prod_pi_i(2))
    ];
    for (from, ans) in test_vec {
      assert_eq!(Expr::from(from), ans);
    }
  }

  #[test]
  fn test_from_shared_rc() {

  }
}