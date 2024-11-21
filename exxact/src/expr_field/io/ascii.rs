use crate::{
  io_traits::AsciiString,
  expr_field::{Expr, structs::Const, FieldTrait, ExprFn}
};

impl AsciiString for Const {
  fn ascii(&self) -> &str {
    &self.ascii
  }
}

impl<'a, Field: FieldTrait<'a>> AsciiString for ExprFn<'a, Field> {
  fn ascii(&self) -> &str {
    match self {
      ExprFn::Sqrt(_) => "Sqrt",
      ExprFn::Sin(_) => "Sin",
      ExprFn::Cos(_) => "Cos",
      ExprFn::Tan(_) => "Tan",
    }
  }
}

// impl AsciiString for StrExpr {
//   fn ascii(&self) -> &str {
//     &self.ascii
//   }
// }

impl<'a, Field: FieldTrait<'a>> AsciiString for Expr<'a, Field> {
  fn ascii(&self) -> &str {
    match self {
      Expr::Zero(_) => {"O"},
      Expr::One(_) => {"I"},
      Expr::InDet(_) => {"?"},
      Expr::Infty(_, _) => {"Infty"},
      Expr::Val(_) => {"Val"},
      Expr::Const(r) => {Field::get_const(*r).ascii},
      Expr::Sum(_) => {"Sum"},
      Expr::Prod(_) => {"Prod"},
      Expr::Fn(r) => {
        todo!()
        // Field::get_fn(*r).ascii()
      },
    }
  }
}

pub trait AsciiChar {
  fn asc_ch(&self) -> char;
}

impl<'a> AsciiChar for Const {
  fn asc_ch(&self) -> char {
    self.ascii.chars().next().unwrap()
  }
}

