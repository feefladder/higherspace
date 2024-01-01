
use crate::enums::{Expr};
use std::rc::Rc;
use std::ops::{Add, Mul};
struct Prod {
  factors: Vec<(Rc<Expr>, i64)>
}

// impl Mul<Prod> for Prod {
//   type Output = Rc<Expr>;
//   fn mul(self, rhs: Prod) -> Rc<Expr> {
      
//   }
// }