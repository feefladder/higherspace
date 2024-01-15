use fraction::Sign;
// use appendlist::AppendList;
use std::{
  cell::{OnceCell, Ref},
  fmt::{
    Debug,
    Display
  },
  ops::{
    Add,
    AddAssign,
    Neg,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
  },
};
use num_traits::{
  Zero,
  One,
  Inv,
  Float,
};

use crate::{
  F,
  expr_field::structs::{Const,Prod,Sum}
};


pub type PVec<'a, Field> = Vec<(Expr<'a, Field>,F)>;
pub type SVec<'a, Field> = Vec<(F,Expr<'a, Field>)>;

pub mod structs;
pub mod simplify;
pub mod io;

mod ops;
pub use ops::*;
/// Field reference

///```raw
///|ch |ascii  |asc_ch |description |
///|'O'|"O"    |'O'    |Zero        |--\
///|'I'|"I"    |'i'    |One         |  |-- adding these will make 
///|'?'|"Nan"  |'?'    |Not a Number|  |   logic nicer
///|'' |"Infty"|'I'    |Infinity (⨥)|--/
///|'ξ'|"Val"  |'V'    |Value       |
///|π,e|pi,e   |'p','e'|Constant    |
///|'Σ'|"Sum"  |'S'    |Sum         |
///|'Π'|"Prod" |'P'    |Prod        |
///|'^'|"Pow"  |'^'    |Power       |
///|   |Sqrt,Cs|'q','C'|Function    |
///```
#[derive(Debug)]
pub enum Expr<'a, Field: FieldTrait<'a>>
where
  Expr<'a, Field>: ExprTrait<'a, Field>{
  // "fundamentals" <- not added to field
  Zero(&'a Field),
  One(&'a Field),
  InDet(&'a Field),
  Infty(&'a Field,Sign),
  // stored in vectors in the field
  Val(FieldRef<'a, Field>),
  Const(FieldRef<'a, Field>),
  Sum(FieldRef<'a, Field>),
  Prod(FieldRef<'a, Field>),
  // Quot(FieldRef<'a, Field>,FieldRef<'a, Field>),
  // Pow(FieldRef<'a, Field>, FieldRef<'a, Field>),
  // 
  Fn(FieldRef<'a, Field>),
}

impl<'a, Field: FieldTrait<'a>> Copy for Expr<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> Clone for Expr<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

/// A reference to the field. On its own, it doesn't do anything.
#[derive(Debug)]
pub struct FieldRef<'a, Field> {
  field: &'a Field,
  index: usize,
}

impl<'a, Field: FieldTrait<'a>> Copy for FieldRef<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> Clone for FieldRef<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

/// Float functions
#[derive(Debug, PartialEq)]
pub enum ExprFn<'a, Field: FieldTrait<'a>>
where{
  Sqrt(Expr<'a, Field>),
  Sin(Expr<'a, Field>),
  Cos(Expr<'a, Field>),
  Tan(Expr<'a, Field>),
}

impl<'a, Field: FieldTrait<'a>> Copy for ExprFn<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> Clone for ExprFn<'a, Field> {
  fn clone(&self) -> Self {
    *self
  }
}


pub trait ExprFnTrait<'a, Field: FieldTrait<'a>>:
{
  fn inner(&self) -> Expr<'a, Field>;
  fn set_inner(&mut self, e: Expr<'a, Field>);
}

impl<'a, Field: FieldTrait<'a>> ExprFnTrait<'a, Field> for ExprFn<'a, Field> {
  fn inner(&self) -> Expr<'a, Field> {
    match self {
      ExprFn::Sqrt(e) => *e,
      ExprFn::Sin(e) => *e,
      ExprFn::Cos(e) => *e,
      ExprFn::Tan(e) => *e,
    }
  }

  fn set_inner(&mut self, e_new: Expr<'a, Field>) {
    match self {
      ExprFn::Sqrt(e) => {*e = e_new},
      ExprFn::Sin(e) => {*e = e_new},
      ExprFn::Cos(e) => {*e = e_new},
      ExprFn::Tan(e) => {*e = e_new},
    }
  }
}

pub trait FieldTrait<'a>:
  // idk
  Sized +
  Debug +
  // Display +
  PartialEq +
  // Eq
{
  /// Create a new Field
  // fn new() -> Self;
  /// Add all elements of the expr to field
  fn gulp(&'a self, expr: Expr<'a, Self>) -> Expr<'a, Self>;

  fn get_val(r: FieldRef<'a, Self>) -> &'a F;
  fn get_const(r: FieldRef<'a, Self>) -> &'a Const;
  fn get_sum(r: FieldRef<'a, Self>) -> &'a Sum<'a, Self>;
  fn get_prod(r: FieldRef<'a, Self>) -> &'a Prod<'a, Self>;
  fn get_fn(r: FieldRef<'a, Self>) -> &'a ExprFn<'a, Self>;

  /// Add a raw sum vector to the field. Should implement the following simplifications:
  /// ```raw
  /// Σ() -> O
  /// Σ(1,I) -> I
  /// Σ(v,I) -> ξv
  /// ```
  fn add_svec(&'a self, s: SVec<'a, Self>) -> Expr<'a, Self>;
  /// Add a raw product vector to the field. Should implement the following:
  /// ```raw
  /// Π() -> I
  /// Π(ξv,1) -> ξv
  /// Π[(ξv,1),...] -> Σ(v,Π[...]) <- this is ugly and adds unnecessary ξv to the field
  /// ```
  fn add_pvec(&'a self, p: PVec<'a, Self>) -> Expr<'a, Self>;
  // Add an expression <- should actually add a thingy, since expressions contain the ref already bla
  // fn add(expr)
  // get the index of an expression.
  // returns None if the expression
  // does not exist within the field.
  // *does* return something if the expr
  // references another field but is in this field
  // fn index_of(&self, expr: ExType) -> Option<usize>;
  // Get the index, append if it doesn't exist within this field
  // fn i_or_add(&self, expr: ExType) -> usize;
  // fn add(&self, expr: ExType) -> usize;
  fn parse(input: &str) -> Expr<'a, Self>;
}

pub trait ExprTrait<'a, Field: FieldTrait<'a>>:
  Sized +
  Clone +
  Copy +
  Debug +
  Display +
  PartialEq +
  // Zero +
  // One +
  Add +
  // AddAssign +
  // Neg +
  // Sub +
  // SubAssign +
  Mul +
  // MulAssign +
  // Inv +
  // Div +
  // DivAssign +
  // Pow<DType> +
  // Float +
{
  fn field(&self) -> &'a Field;
  // fn set_ref_to(&mut self, field: Field);
}

impl<'a, F1: FieldTrait<'a>, F2: FieldTrait<'a>> PartialEq<Expr<'a, F1>> for Expr<'a, F2> {
  fn eq(&self, other: &Expr<'a, F1>) -> bool {
    // if &self.field() == &other.field() && 
    // match (self, other) {
    //   (Expr::
    // }
    todo!()
  }
}

impl<'a, Field: FieldTrait<'a>> Display for Expr<'a, Field> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl<'a, Field: FieldTrait<'a>> ExprTrait<'a, Field> for Expr<'a, Field> {
  fn field(&self) -> &'a Field {
    match *self {
      Expr::Zero(f) => f,
      Expr::One(f) => f,
      Expr::InDet(f) => f,
      Expr::Infty(f, _) => f,
      Expr::Val(r) => r.field,
      Expr::Const(r) => r.field,
      Expr::Sum(r) => r.field,
      Expr::Prod(r) => r.field,
      Expr::Fn(r) => r.field,
    }
  }
}