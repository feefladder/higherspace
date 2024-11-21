
use crate::enums::{Expr};
use std::rc::Rc;
use std::ops::{Add, Mul};

/// A product of factors. Note that pi²=pi*pi is a product of factors.
struct Prod {
  factors: Vec<(Expr, i64)>
}

// impl Mul<Prod> for Prod {
//   type Output = Rc<Expr>;
//   fn mul(self, rhs: Prod) -> Rc<Expr> {
      
//   }
// }