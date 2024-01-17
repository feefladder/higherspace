// use std::rc::Rc;

use std::{hash::Hash, ops::{Deref, DerefMut}};
use ordered_float::NotNan;
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

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Const {
  pub ch: char,
  pub ascii: &'static str,
  pub f64: NotNan<f64>,
}

impl PartialEq for Const {
  fn eq(&self, other: &Self) -> bool {
    self.f64 == other.f64
  }
}

#[derive(Debug)]
pub struct Sum<'a, Field: FieldTrait<'a>>{
  pub terms: Vec<(F, Expr<'a, Field>)>
}

impl<'a, Field: FieldTrait<'a>> Hash for Sum<'a, Field> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for t in &self.terms {
      t.0.hash(state);
      // calls hash recursively
      t.1.hash(state);
    }
  }
}

impl<'a, Field: FieldTrait<'a>> Hash for Prod<'a, Field> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for t in &self.factors {
      t.0.hash(state);
      // calls hash recursively
      t.1.hash(state);
    }
  }
}

impl<'a,Field: FieldTrait<'a>> Clone for Sum<'a, Field> {
  fn clone(&self) -> Self {
    Sum { terms: self.terms.clone() }
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

trait UnorderedEq {
  fn unordered_eq(&self, other: &Self) -> bool;
}

impl<T: Eq + Clone> UnorderedEq for Vec<T> {
  fn unordered_eq(&self, other: &Self) -> bool {
      if self.len() != other.len() {
          return false;
      }

      let mut match_count = 0;

      let mut other_to_check: Self = Vec::new();
      other_to_check.clone_from(other);

      for item in self {
          let index_in_other = other_to_check.iter().position(|e| e == item);

          if let Some(index) = index_in_other {                
              other_to_check.remove(index);
              match_count += 1;
          }
      }

      self.len() == match_count
  }
}
