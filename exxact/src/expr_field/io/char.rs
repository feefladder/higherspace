use crate::{
  expr_field::{
    // Val,
    Const,
    // Fn, 
    Expr, FieldTrait, ExprFn,
    // structs::StrExpr,
  },
  io_traits::Char
};

impl Char for Const {
  fn ch(&self) -> char {
    self.ch
  }
}

// impl Char for StrExpr {
//   fn ch(&self) -> char {
//     self.ch()
//   }
// }

impl<'a, Field: FieldTrait<'a>> Char for ExprFn<'a, Field> {
  fn ch(&self) -> char {
    match self {
      ExprFn::Sqrt(_) => '√',
      ExprFn::Sin(_) => '🕢',
      ExprFn::Cos(_) => '🕑',
      ExprFn::Tan(_) => '🕘',
    }
  }
}

impl<'a, Field: FieldTrait<'a>> Char for Expr<'a, Field> {
  fn ch(&self) -> char {
    match self {
      Expr::Zero(_) => {'O'},
      Expr::One(_) => {'I'},
      Expr::InDet(_) => {'?'},
      Expr::Infty(_, _) => {'∞'},
      Expr::Val(_) => {'ξ'},
      Expr::Const(r) => {Field::get_const(*r).ch()},
      Expr::Sum(_) => {'Σ'},
      Expr::Prod(_) => {'Π'},
      Expr::Fn(r) => {Field::get_fn(*r).ch()},
    }
  }
}