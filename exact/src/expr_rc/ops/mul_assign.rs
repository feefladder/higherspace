use std::ops::MulAssign;

use crate::expr_rc::Expr;

impl MulAssign for Expr {
  fn mul_assign(&mut self, rhs: Self) {
      *self = self.clone() * rhs;
  }
}