use std::fmt::{Display, Debug};
use num::abs;
use num::{One, Zero};

use crate::{
  expr_field::{
    structs::{Prod, Sum}, Expr, ExprFn, ExprFnTrait, ExprTrait, FieldTrait, Sign
  }, io_traits::Char, F};


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

fn fmt_prod_tup<Field: FieldTrait>(f: &mut std::fmt::Formatter<'_>, base: Expr::<'_, Field>, exp: F) -> std::fmt::Result {
  write!(f, "{}", base)?;
  match exp {
    F::Rational(s, r) if exp.is_one()  => {
      write!(f, "{}", r.numer()
    .to_string()
    .chars()
    .map(|c| match c {
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '0' => '⁰',
        _ => '?',
    })
    .collect::<String>())
    }
    _ => write!(f,"^({})",exp.get_unicode_display()),
  }
}

fn fmt_sum_tup<Field: FieldTrait>(f: &mut std::fmt::Formatter<'_>, coeff: F, factor: Expr<'_, Field>) -> std::fmt::Result {
  if matches!(factor, Expr::One(_)) {
    write!(f, "{}", coeff.get_unicode_display())
  } else if coeff.is_one() {
      write!(f, "{}", factor)
  } else {
    write!(f, "{}{}", coeff.get_unicode_display(), factor)
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for Sum<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl <'a, F: FieldTrait<'a>> Display for Sum<'a, F>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let v= &self.terms;
    if v.len() == 0 {
      write!(f, "()")
    } else if v.len() == 1 {
      write!(f,"({}{})",v[0].0.get_unicode_display(),v[0].1)
    } else {
      write!(f,"[{}{}",v[0].0.get_unicode_display(),v[0].1)?;
      for (coeff, fact) in &v[1..] {
        write!(f,"+{}{}",coeff.get_unicode_display(),fact)?;
      }
      write!(f, "]")
    }
  }
}

impl<'a, Field: FieldTrait<'a>> Debug for Prod<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}
impl<'a, F: FieldTrait<'a>> Display for Prod<'a, F> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let v= &self.factors;
    if v.len() == 0 {
      write!(f, "()")
    } else if v.len() == 1 {
      write!(f,"({}{})",v[0].1.get_unicode_display(),v[0].0)
    } else {
      write!(f,"[{}{}",v[0].1.get_unicode_display(),v[0].0)?;
      for (coeff, fact) in &v[1..] {
        write!(f,"⁢{}{}",coeff.get_unicode_display(),fact)?;
      }
      write!(f, "]")
    }
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
      Expr::Prod(r) => write!(f, "{}{}", self.ch(), Field::get_prod(*r)),
      Expr::Fn(r) => write!(f, "{}"/*, self.ch()*/, Field::get_fn(*r)),
    }
  }
}

#[cfg(test)]
mod test_exprf_display {
  use super::*;
  use crate::expr_field::{structs::Const, TypeField};
  use fraction::Sign;

  fn test_fundamentals() {
    let f = TypeField::default();
    let tv = vec![
      (Expr::Zero(&f), "O"),
      (f.add_val(F::zero()), "O"),
      (Expr::One(&f), "I"),
      (f.add_val(F::one()), "I"),
      (Expr::InDet(&f), "?"),
      (Expr::Infty(&f, Sign::Plus), "∞"),
      (Expr::Infty(&f, Sign::Minus), "-∞"),
      (f.add_const(Const{ch: 'π', ascii: "π", f64: ordered_float::NotNan::new(std::f64::consts::PI).unwrap()}), "π"),
      
      (f.add_val(F::from(2)), "2"),
      (f.add_val(F::from(0.5)), "1⁄2")
    ];
    for (expr, s) in tv {
      println!("{} ?= {}", expr, s);
      assert_eq!(format!("{}", expr), s);
    }
  }
}

