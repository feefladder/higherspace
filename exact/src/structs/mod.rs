// use std::rc::Rc;

// TODO: generic Expr
use crate::{F, expr_rc::Expr, io::Char};

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

#[derive(Debug, PartialEq, Clone)]
pub struct Sum{
  pub terms: Vec<(F, Expr)>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Prod{
  pub factors: Vec<(Expr, F)>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sqrt(pub Expr);
