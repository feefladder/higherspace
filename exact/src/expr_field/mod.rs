use fraction::Sign;
// use appendlist::AppendList;
use core::{
  cell::Ref,
  ops::{
    Add,
    // AddAssign,
    // Neg,
    // Sub,
    // SubAssign,
    Mul,
    // MulAssign,
    // Div,
    // DivAssign
  },
};
use std::{
  fmt::{
    Debug,
    Display
  },

  cmp::Ordering, hash::Hash, ops::Deref
};
// use num_traits::{
//   Zero,
//   One,
//   Inv,
//   Float,
// };

use crate::{
  F,
  expr_field::structs::{Const,Prod,Sum}, io_traits::Char
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

// impl<'a, Field: FieldTrait<'a>> Hash for Expr<'a, Field> {
  
// }

impl<'a, Field: FieldTrait<'a>> Copy for Expr<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> Clone for Expr<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

impl<'a, Field: FieldTrait<'a>> Hash for Expr<'a, Field> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.ch().hash(state);
    match self {
      Expr::Val(r) => {
        Field::get_val(*r).hash(state)
      },
      Expr::Const(r) => {
        Field::get_const(*r).hash(state)
      },
      Expr::Sum(r) => {
        Field::get_sum(*r).deref().hash(state)
      },
      Expr::Prod(r) => {
        Field::get_prod(*r).deref().hash(state)
      },
      Expr::Fn(r) => {
        Field::get_fn(*r).inner().hash(state)
      },
      Expr::Infty(r, s) => {
        s.hash(state);
        // self.ch().hash(state)
      },
      // For the rest, hashing the ch() should be good enough
      _ => {}
    }
  }
}

/// A reference to the field. On its own, it doesn't do anything.
#[derive(Debug, Eq)]
pub struct FieldRef<'a, Field: FieldTrait<'a>> {
  field: &'a Field,
  index: usize,
}

impl<'a, Field: FieldTrait<'a>> Copy for FieldRef<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> Clone for FieldRef<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

/// very restrictive equality, pointer equality on the Field
/// This prevents us from having to deep-compare the Field
impl<'a, Field: FieldTrait<'a>> PartialEq for FieldRef<'a, Field> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.field, other.field) && self.index == other.index
  }
}

/// Float functions
#[derive(Debug)]
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

impl<'a, Field: FieldTrait<'a>> PartialEq for ExprFn<'a, Field>
{
  fn eq(&self, other: &ExprFn<'a, Field>) -> bool {
    // https://stackoverflow.com/a/32554326/14681457
    if std::mem::discriminant(self) == std::mem::discriminant(other) {
      self.inner() == other.inner()
    } else {
      false
    }
    // This would be needed if the two fields are different
    // match (self, other) {
    //   (ExprFn::Sqrt(_), ExprFn::Sqrt(_)) => self.inner() == other.inner(),
    //   (ExprFn::Sin(_),ExprFn::Sin(_)) => self.inner() == other.inner(),
    //   (ExprFn::Cos(_),ExprFn::Cos(_)) => self.inner() == other.inner(),
    //   (ExprFn::Tan(_),ExprFn::Tan(_)) => self.inner() == other.inner(),
    //   _ => false
    // }
  }
}

// impl<'a, Field: FieldTrait<'a>> Hash for ExprFn<'a, Field> {
//   fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//     self.ch().hash(state);
//     self.inner().hash(state)
//   }
// }

pub trait ExprFnTrait<'a, Field: FieldTrait<'a>>:
  Debug +
  Display + 
  Clone +
  Copy +
  // PartialEq +
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
  // PartialEq +
  // Eq
{
  /// Create a new Field
  // fn new() -> Self;
  /// Add all elements of the expr to field
  fn gulp(&'a self, expr: Expr<'a, Self>) -> Expr<'a, Self>;

  // Val, Const and ExprFn are Copy, so return directly
  fn get_val(r: FieldRef<'a, Self>) -> F;
  fn get_const(r: FieldRef<'a, Self>) -> Const;
  fn get_fn(r: FieldRef<'a, Self>) -> ExprFn<'a, Self>;
  // Sum and Prod have a Vec which we may not want to clone
  fn get_sum(r: FieldRef<'a, Self>) -> Ref<Sum<'a, Self>>;
  fn get_prod(r: FieldRef<'a, Self>) -> Ref<Prod<'a, Self>>;
  

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
  fn add_val(&'a self, v: F) -> Expr<'a, Self>;
  fn add_const(&'a self, c: Const) -> Expr<'a, Self>;
  fn add_fn(&'a self, f: ExprFn<'a, Self>) -> Expr<'a, Self>;
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
  fn parse(&'a self, input: &str) -> Expr<'a, Self>;
}

pub trait ExprTrait<'a, Field: FieldTrait<'a>>:
  Sized +
  Clone +
  Copy +
  Debug +
  Display +
  // PartialEq +
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
  // fn get_ref(&self) -> &'a FieldRef<'a, Field>;
  // fn set_ref_to(&mut self, field: Field);
}

impl<'a, Field: FieldTrait<'a>> Eq for Expr<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> PartialEq for Expr<'a, Field> where
{
  fn eq(&self, other: &Expr<'a, Field>) -> bool {
    match (self, other) {
      // "fundamentals"
      (Expr::Zero(_), Expr::Zero(_)) => true,
      (Expr::One(_), Expr::One(_)) => true,
      //Nan != Nan, but we want Eq trait, so InDet == Indet
      (Expr::InDet(_), Expr::InDet(_)) => true,
      (Expr::Infty(_, s1), Expr::Infty(_, s2)) => {s1 == s2}
      (Expr::Val(r1), Expr::Val(r2)) => {
        Field::get_val(*r1) == Field::get_val(*r2)
      },
      (Expr::Const(r1), Expr::Const(r2)) => {
        Field::get_const(*r1) == Field::get_const(*r2)
      },
      (Expr::Sum(r1), Expr::Sum(r2)) => {
        if r1 == r2 {
          true
        } else {
          *Field::get_sum(*r1) == *Field::get_sum(*r2)
        }
      },
      (Expr::Prod(r1), Expr::Prod(r2)) => {
        if r1 == r2 {
          true
        } else {
          *Field::get_prod(*r1) == *Field::get_prod(*r2)
        }
      },
      (Expr::Fn(r1), Expr::Fn(r2)) => {
        Field::get_fn(*r1) == Field::get_fn(*r2)
      }
      _ => false
    }
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

// impl<Field> Ord for Expr<'a, Field> {
//   fn cmp(&self, other: &Self) -> Ordering {
//     match (self, other) {
//       (Expr::Infty(_, s1), Expr::Infty(_, s2)) => {
//         if s1 == s2 { Ordering::Equal } else { s1 < s2 }
//       }
//       (Expr::Zero(_), Expr::Zero(_)) => Ordering::Equal,
//       (Expr::)
//     }
//   }
// }