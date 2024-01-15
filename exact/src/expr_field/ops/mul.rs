use std::ops::Mul;

use crate::expr_field::{Expr, FieldTrait};

impl<'a, Field: FieldTrait<'a>> Mul for Expr<'a, Field> {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    todo!()
  }
}