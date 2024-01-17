use std::collections::HashSet;
use crate::expr_field::{structs::Prod, FieldTrait};

use super::UnorderedEq;

impl<'a, Field: FieldTrait<'a>> Eq for Prod<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> PartialEq for Prod<'a, Field> {
  fn eq(&self, other: &Prod<'a, Field>) -> bool {
    // self.factors.unordered_eq(&other.factors)
    let s: HashSet<_> = self.factors.iter().collect();
    let o: HashSet<_> = other.factors.iter().collect();
    s == o
  }
}

// impl Mul<Prod> for Prod {
//   type Output = Rc<Expr>;
//   fn mul(self, rhs: Prod) -> Rc<Expr> {
      
//   }
// }