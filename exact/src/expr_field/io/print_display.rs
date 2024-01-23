use std::fmt::{Display, Debug};
use crate::{
  expr_field::{
    Sign,
    Expr,
    ExprFn, FieldTrait, ExprFnTrait,
    structs::{Sum, Prod}, ExprTrait
  }, io_traits::Char};


/// Format a tuple vec such as SVec or PVec
fn fmt_tuple_vec<A: Display, B: Display>(v: &Vec<(A,B)>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  if v.len() == 0 {
    write!(f, "()")
  } else if v.len() == 1 {
    write!(f,"({},{})",v[0].0,v[0].1)
  } else {
    write!(f,"[({},{})",v[0].0,v[0].1)?;
    for (a,b) in &v[1..] {
      write!(f,",({},{})",a,b)?;
    }
    write!(f,"]")
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for Sum<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl <'a, F: FieldTrait<'a>> Display for Sum<'a, F>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    fmt_tuple_vec(&self.terms, f)
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for Prod<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl<'a, F: FieldTrait<'a>> Display for Prod<'a, F> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    fmt_tuple_vec(&self.factors, f)
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for ExprFn<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl<'a, Field: FieldTrait<'a>> Display for ExprFn<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.inner() {
      Expr::Val(r) => {
        write!(f,"{}{}", self.ch(), Field::get_val(r))
      },
      _ => write!(f, "{}{}", self.ch(), self.inner())
    }
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for Expr<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:p}{}",self.field(), self)
  }
}
impl<'a, Field: FieldTrait<'a>> Display for Expr<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expr::Zero(_) => write!(f, "{}", self.ch()),
      Expr::One(_) => write!(f, "{}",self.ch()),
      Expr::InDet(_) => write!(f, "{}", self.ch()),
      Expr::Infty(_, s) => write!(f, "{}{}", if *s == Sign::Minus {"-"} else {""}, self.ch()),
      Expr::Const(_) => write!(f, "{}", self.ch()),
      Expr::Val(r) => write!(f, "{}"/*, self.ch()*/, Field::get_val(*r)),
      Expr::Sum(r) => write!(f, "{}{}", self.ch(), Field::get_sum(*r)),
      Expr::Prod(r) => {
        write!(f, "{}", self.ch())?;
        let p = Field::get_prod(*r);
        if p.factors.len() == 1 {
          let (base, exp) = &p.factors.first().unwrap();
          write!(f,"({},{})",base,exp)
        } else {
          let (f0, e0) = &p.factors[0];
          // opening bracket and first term for comma stuff
          write!(f,"[({},{})", f0, e0)?;
          for (base, exp) in &p.factors[1..] {
            write!(f,",({},{})",base, exp)?;
          }
          write!(f,"]")
        }
      },
      Expr::Fn(r) => write!(f, "{}"/*, self.ch()*/, Field::get_fn(*r)),
    }
  }
}