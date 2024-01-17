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
// #[derive(Debug)]
pub enum Expr<'a, Field>
{
  // "fundamentals" <- not added to field
  InDet(&'a Field),
  Infty(&'a Field,Sign),
  Zero(&'a Field),
  One(&'a Field),
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

impl<'a, Field> Copy for Expr<'a, Field> {}
impl<'a, Field> Clone for Expr<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

// impl<'a, Field: FieldTrait<'a>> Hash for Expr<'a, Field> {
//   fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//     self.ch().hash(state);
//     match self {
//       Expr::Val(r) => {
//         Field::get_val(*r).hash(state)
//       },
//       Expr::Const(r) => {
//         Field::get_const(*r).hash(state)
//       },
//       Expr::Sum(r) => {
//         Field::get_sum(*r).deref().hash(state)
//       },
//       Expr::Prod(r) => {
//         Field::get_prod(*r).deref().hash(state)
//       },
//       Expr::Fn(r) => {
//         Field::get_fn(*r).inner().hash(state)
//       },
//       Expr::Infty(r, s) => {
//         s.hash(state);
//         // self.ch().hash(state)
//       },
//       // For the rest, hashing the ch() should be good enough
//       _ => {}
//     }
//   }
// }

#[derive(Debug)]
pub enum TestMe<'a, Field> {
  Smaller(&'a Field),
  Larger(TestRef<'a, Field>)
}

#[derive(Debug)]
pub struct TestRef<'a, Field> {
  f: &'a Field,
  i: usize,
}

impl<'a, Field> Copy for TestRef<'a, Field>{}
impl<'a, Field> Clone for TestRef<'a, Field>{
  fn clone(&self) -> Self {
    *self
  }
}
impl<'a, Field> Copy for TestMe<'a, Field>{}
impl<'a, Field> Clone for TestMe<'a, Field>{
  fn clone(&self) -> Self {
    *self
  }
}
impl<'a, Field> Eq for TestMe<'a, Field>{}
impl<'a, Field> PartialEq for TestMe<'a, Field> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (TestMe::Smaller(rs),TestMe::Smaller(ro)) => std::ptr::eq(rs,ro),
      (TestMe::Larger(rs),TestMe::Larger(ro)) => std::ptr::eq(rs,ro),
      _ => false
    }
  }
}
impl<'a, Field> PartialOrd for TestMe<'a, Field> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl<'a, Field> Ord for TestMe<'a, Field> {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (TestMe::Smaller(_), TestMe::Smaller(_)) => Ordering::Equal,
      (TestMe::Smaller(_), _) => Ordering::Less,
      (TestMe::Larger(_),TestMe::Larger(_)) => Ordering::Equal,
      (TestMe::Larger(_),_) => Ordering::Greater,
    }
  }
}

/// A reference to the field. On its own, it doesn't do anything.
#[derive(Eq)]
pub struct FieldRef<'a, Field> {
  pub field: &'a Field,
  index: usize,
}

impl<'a, Field> Copy for FieldRef<'a, Field> {}
impl<'a, Field> Clone for FieldRef<'a, Field> {
  fn clone(&self) -> Self {
      *self
  }
}

/// very restrictive equality, pointer equality on the Field
/// This prevents us from having to deep-compare the Field
/// if the Field is not the same, we'll resort to recursive comparison of exprs
impl<'a, Field> PartialEq for FieldRef<'a, Field> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.field, other.field) && self.index == other.index
  }
}

/// Float functions
// #[derive(Debug)]
pub enum ExprFn<'a, Field>
where{
  Sqrt(Expr<'a, Field>),
  Sin(Expr<'a, Field>),
  Cos(Expr<'a, Field>),
  Tan(Expr<'a, Field>),
}

impl<'a, Field> Copy for ExprFn<'a, Field> {}
impl<'a, Field> Clone for ExprFn<'a, Field> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, Field> PartialEq for ExprFn<'a, Field> where
  Expr<'a, Field>: PartialEq
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

pub trait ExprFnTrait<'a, Field>:
  // Debug +
  // Display + 
  Clone +
  Copy +
  // PartialEq +
{
  fn inner(&self) -> Expr<'a, Field>;
  fn set_inner(&mut self, e: Expr<'a, Field>);
}

impl<'a, Field> ExprFnTrait<'a, Field> for ExprFn<'a, Field> {
  #[inline]
  fn inner(&self) -> Expr<'a, Field> {
    match self {
      ExprFn::Sqrt(e) => *e,
      ExprFn::Sin(e) => *e,
      ExprFn::Cos(e) => *e,
      ExprFn::Tan(e) => *e,
    }
  }

  #[inline]
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
  /// Σ[(1,π),(1,I)] -> Σ[(1,I),(1,π)]
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

pub trait ExprTrait<'a, Field>:
  Sized +
  Clone +
  Copy +
  // Debug +
  // Display +
  // PartialEq +
  // Zero +
  // One +
  // Add +
  // AddAssign +
  // Neg +
  // Sub +
  // SubAssign +
  // Mul +
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
impl<'a, Field: FieldTrait<'a>> PartialEq for Expr<'a, Field>
{
  #[inline]
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

impl<'a, Field> ExprTrait<'a, Field> for Expr<'a, Field> {
  #[inline]
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


impl<'a, Field: FieldTrait<'a>> PartialOrd for Expr<'a, Field> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
/// Ordering that determines how things are sorted in sums and prods.
/// '?'<'∞'<'O'<'I'<'ξ'<'const'<'Σ'<'Π'<fn
/// This does NOT compare the "value" of Exprs
/// e.g. O<-1
impl<'a, Field: FieldTrait<'a>> Ord for Expr<'a, Field> {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Expr::InDet(_),Expr::InDet(_)) => Ordering::Equal,
      (Expr::InDet(_),_) => Ordering::Less,
      (_,Expr::InDet(_)) => Ordering::Greater,
      (Expr::Infty(_, s1),Expr::Infty(_, s2)) => s1.cmp(s2),
      (Expr::Infty(_, _),_) => Ordering::Less,
      (_,Expr::Infty(_,_)) => Ordering::Greater,
      (Expr::Zero(_),Expr::Zero(_)) => Ordering::Equal,
      (Expr::Zero(_),_) => Ordering::Less,
      (_,Expr::Zero(_)) => Ordering::Greater,
      (Expr::One(_),Expr::One(_)) => Ordering::Equal,
      (Expr::One(_),_) =>  Ordering::Less,
      (_,Expr::One(_)) =>  Ordering::Greater,
      (Expr::Val(rs),Expr::Val(ro)) => {
        Field::get_val(*rs).cmp(&Field::get_val(*ro))
      },
      (Expr::Val(_),_) => Ordering::Less,
      (_,Expr::Val(_)) => Ordering::Greater,
      // sort constants "alphabetically"
      (Expr::Const(_),Expr::Const(_)) => {
        self.ch().cmp(&other.ch())
      },
      (Expr::Const(_),_) => Ordering::Less,
      (_,Expr::Const(_)) => Ordering::Greater,
      // Sums SHOULD be sorted
      // otherwise 1+π < π+1
      (Expr::Sum(rs),Expr::Sum(ro)) => {
        Field::get_sum(*rs).cmp(&Field::get_sum(*ro))
      },
      (Expr::Sum(_),_) => Ordering::Less,
      (_,Expr::Sum(_)) => Ordering::Greater,
      (Expr::Prod(rs),Expr::Prod(ro)) => {
        Field::get_prod(*rs).cmp(&Field::get_prod(*ro))
      },
      (Expr::Prod(_),_) => Ordering::Less,
      (_,Expr::Prod(_)) => Ordering::Greater,
      (Expr::Fn(rs),Expr::Fn(ro)) => {
        if self.ch() == other.ch() {
          Field::get_fn(*rs).inner().cmp(&Field::get_fn(*ro).inner())
        } else {
          self.ch().cmp(&other.ch())
        }
      },
      (Expr::Fn(_),_) => Ordering::Greater,
      _ => Ordering::Equal,
    }
  }
}

#[cfg(test)]
mod test_expr_field {
  use crate::{F, One};
  use super::{Expr, structs::type_field::{TypeField, TypeExpr}, FieldTrait, TestMe};

  #[test]
  fn test_order_strict() {
    let tv = vec![
      "?",
      "-∞",
      "∞",
      "O",
      "I",
      "-1",
      "3/2",
      "2",
      "5",
      "e",
      "π",
      "Σ(2,π)",
      "Π(π,2)",
      "√5"
    ];
    let f =  TypeField::default();
    for i in 0..tv.len() {
      for j in i+1..tv.len() {
        println!("testing {}<{}",tv[i],tv[j]);
        println!("testing {}<{}",f.parse(tv[i]),f.parse(tv[j]));
        assert!(f.parse(tv[i]) < f.parse(tv[j]));
      }
    }
    let mut v: Vec<TypeExpr<'_>> = tv.iter().map(|a| f.parse(a)).collect();
    v.sort_unstable_by(|a,b|b.cmp(a));
    for i in 0..v.len() {
      for j in i+1..v.len() {
        println!("testing {}>{}",v[i],v[j]);
        println!("testing {}>{}",v[i],v[j]);
        assert!(v[i] > v[j]);
      }
    }
    // assert!(false)
  }

  #[test]
  fn testme_order_strict() {
    let f =  TypeField::default();
    let tv = vec![
      TestMe::Smaller(&f),
      TestMe::Larger(super::TestRef { f: &f, i: 0 })
    ];
    
    for i in 0..tv.len() {
      for j in i+1..tv.len() {
        println!("testing {:?}<{:?}",tv[i],tv[j]);
        println!("testing {:?}<{:?}",tv[i],tv[j]);
        assert!(tv[i] < tv[j]);
      }
    }
    // let mut v: Vec<TypeExpr<'_>> = tv.iter().map(|a| f.parse(a)).collect();
    let mut v = tv;
    v.sort_unstable_by(|a,b|b.cmp(a));
    for i in 0..v.len() {
      for j in i+1..v.len() {
        println!("testing {:?}>{:?}",v[i],v[j]);
        println!("testing {:?}>{:?}",v[i],v[j]);
        assert!(v[i] > v[j]);
      }
    }
    let mut sv:Vec<(F, TestMe<'_, TypeField<'_>>)> = v.iter().map(|e| (F::from(1), *e)).collect();
    sv.sort_unstable();
    for i in 0..v.len() {
      for j in i+1..v.len() {
        println!("testing {:?}<{:?}",sv[i].1,sv[j].1);
        println!("testing {:?}<{:?}",sv[i].1,sv[j].1);
        assert!(sv[i] < sv[j]);
      }
    }
    // assert!(false)
  }
}