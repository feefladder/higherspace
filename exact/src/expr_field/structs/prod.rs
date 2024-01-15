
use crate::expr_field::{structs::Prod, FieldTrait};

impl<'a, Field: FieldTrait<'a>> PartialEq for Prod<'a, Field> {
  fn eq(&self, other: &Self) -> bool {
    // this is ugly
    for i in self.factors.clone() {
      if !other.factors.contains(&i) {return false;}
    }
    for i in other.factors.clone() {
      if !self.factors.contains(&i) {return false;}
    }
    true
  }
}

// impl Mul<Prod> for Prod {
//   type Output = Rc<Expr>;
//   fn mul(self, rhs: Prod) -> Rc<Expr> {
      
//   }
// }