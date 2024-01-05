// use std::rc::Rc;

// TODO: generic Expr
use crate::{F, expr_rc::Expr, io::{Char,ParseDisplay}};

// pub mod consts;
// pub mod sqrt;
// pub mod prod;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Const {
  pub ch: char,
  pub f64: f64,
}

impl Char for Const {
  fn ch(&self) -> char {
    self.ch
  }
}

#[derive(Debug, Clone)]
pub struct Sum{
  pub terms: Vec<(F, Expr)>
}

impl PartialEq for Sum {
  fn eq(&self, other: &Self) -> bool {
    // this is ugly
    for i in self.terms.clone() {
      if !other.terms.contains(&i) {return false;}
    }
    for i in other.terms.clone() {
      if !self.terms.contains(&i) {return false;}
    }
    true
  }
}

#[derive(Debug, Clone)]
pub struct Prod{
  pub factors: Vec<(Expr, F)>
}

impl PartialEq for Prod {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Sqrt(pub Expr);

#[test]
fn test_set_eq() {
  let v = vec![
    ("Σ[(2,π),(1,ξ1)]","Σ[(1,ξ1),(2,π)]"),
    ("Π[(π,2),(e,1)]","Π[(e,1),(π,2)]")
  ];
  for (a,b) in v {
    assert_eq!(
      Expr::parse_display(a.to_string()),
      Expr::parse_display(b.to_string())
    );
  }
}