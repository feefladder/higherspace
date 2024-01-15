// use std::rc::Rc;

// TODO: generic Expr
use crate::{
  F,
  expr_field::{
    FieldTrait, Expr
  }
};

pub mod sum;
pub mod prod;
pub mod type_field;

pub mod string_expression;
pub use string_expression::*;



#[derive(Debug, PartialEq, Clone)]
pub struct Const {
  pub ch: char,
  pub ascii: &'static str,
  pub f64: f64,
}


#[derive(Debug)]
pub struct Sum<'a, Field: FieldTrait<'a>>{
  pub terms: Vec<(F, Expr<'a, Field>)>
}

impl<'a,Field: FieldTrait<'a>> Clone for Sum<'a, Field> {
  fn clone(&self) -> Self {
    Sum { terms: self.terms.clone() }
  }
}

impl<'a, Field: FieldTrait<'a>> PartialEq for Sum<'a, Field> {
  fn eq(&self, other: &Self) -> bool {
    // this is ugly
    for i in &self.terms {
      if !other.terms.contains(&i) {return false;}
    }
    for i in &other.terms {
      if !self.terms.contains(&i) {return false;}
    }
    true
  }
}

#[derive(Debug)]
pub struct Prod<'a, Field: FieldTrait<'a>>{
  pub factors: Vec<(Expr<'a, Field>, F)>
}

impl<'a, Field: FieldTrait<'a>> Clone for Prod<'a, Field> {
  fn clone(&self) -> Self {
    Prod { factors: self.factors.clone() }
  }
}
