use std::rc::Rc;
use std::f64::consts::{PI,E};

use crate::F;
use crate::structs::{Const, Sum, Prod, Sqrt};

mod from;
mod mul;
mod add;
mod inv;
pub use from::*;
pub use mul::*;
pub use add::*;
pub use inv::*;

// #[derive(Debug, PartialEq, Clone)]
// pub enum VOrRef {
//   Val(F),
//   Ref(Rc<Expr>),
// }

/// Expression using Rc<T> This type does not know if there are duplicates
/// use like:
/// let half = F::new(1u8,2u8); // <- for convenience
/// let val_1 = Expr::from(1u32);
/// let val_5 = Expr::from(5u32);
/// let sqrt_5 = Expr::sqrt(val_5);
/// let phi = Expr::from(
///   vec![(half,val_1),(half,sqrt_5)]
/// )
/// let phi_sq = Expr::from(
///   vec![(F::new(3u8,2u8),val_1),(half,sqrt_5)]
/// )
/// to be sure that sqrt_5 is shared among phi and phi_sq
#[derive(PartialEq, Clone)]
pub enum Expr {
  /// Just a number
  /// Needs to be there, since sqrt(2)*sqrt(2) = 2
  Val(Rc<F>),
  /// Constant like pi
  Const(Rc<Const>),
  /// Sum of terms
  /// Note that
  Sum(Rc<Sum>),
  /// Product of factors
  /// Notice that e.g. 2pi is already a product
  Prod(Rc<Prod>),
  Sqrt(Rc<Sqrt>),
}

impl Expr {
  #[inline]
  pub fn val_i<T>(i: T) -> Expr where F: From<T>{
    Expr::Val(Rc::new(F::from(i)))
  }
  #[inline]
  pub fn val_frac(f: F) -> Expr {
    Expr::Val(Rc::new(f))
  }
  #[inline]
  pub fn c_pi() -> Expr{
    Expr::Const(Rc::new(Const { ch: 'π', f64: PI }))
  }
  #[inline]
  pub fn c_e() -> Expr{
    Expr::Const(Rc::new(Const { ch: 'e', f64: E }))
  }
  #[inline]
  pub fn sqrt_i(i: u32) -> Expr {
    Expr::Sqrt(Rc::new(Sqrt(Expr::val_i(i))))
  }
  #[inline]
  pub fn sqrt_frac(f: F) -> Expr {
    Expr::Sqrt(Rc::new(Sqrt(Expr::from(f))))
  }
  #[inline]
  pub fn sqrt_expr(expr: Expr) -> Expr {
    Expr::Sqrt(Rc::new(Sqrt(expr)))
  }
  #[inline]
  pub fn sum_i_pi(i: u32) -> Expr {
    Expr::Sum(Rc::new(Sum{
      terms : vec![
        (F::from(i),Expr::c_pi())
      ]
    }))
  }
  #[inline]
  pub fn sum_i_expr(i:u32, e: Expr) -> Expr {
    Expr::Sum(Rc::new(Sum{
      terms : vec![
        (F::from(i),e)
      ]
    }))
  }
  #[inline]
  pub fn sum_i_plus_pi(i: u32) -> Expr {
    Expr::Sum(Rc::new(Sum { 
      terms: vec![
        (F::from(i),Expr::val_i(1)),
        (F::from(1),Expr::c_pi()),
      ]
     }))
  }
  /// the golden ratio: (1+sqrt(5))/2 = 1/2+1/2sqrt(5)
  #[inline]
  pub fn sum_phi() -> Expr {
    Expr::Sum(Rc::new(Sum { terms: vec![
      (F::new(1u8, 2u8), Expr::val_i(1)),
      (F::new(1u8, 2u8), Expr::sqrt_i(5))
    ] }))
  }
  #[inline]
  pub fn prod_pi_i(i:u32) -> Expr {
    Expr::Prod(Rc::new(Prod{factors:vec![
      (Expr::c_pi(),F::from(i)),
    ]}))
  }
  #[inline]
  pub fn prod_pi_times_sqrt_i(i: u32) -> Expr {
    Expr::Prod(Rc::new(Prod { factors: vec![
      (Expr::c_pi(),F::from(1)),
      (Expr::sqrt_i(i),F::from(1))
    ] }))
  }
}

/// Get the expression of a functional expression, such as Sqrt, Sin, Cos, Tan
/// Currently only sqrt. 
/// √5/2 -> ξ5/2
/// Expr::Sqrt(Rc<Sqrt(expr)>) -> expr
pub trait GetExpr {
  fn get_expr(&self) -> Option<Expr>;
}

impl GetExpr for Expr {
  fn get_expr(&self) -> Option<Expr> {
    match &self {
        Expr::Sqrt(s) => {
          Some(s.as_ref().clone().0)
        },
        _ => {
          None
        }
    }
  }
}




// impl Mul<Expr> for &mut Rc<Expr> {
//   // pub mod sqrt;
//   // pub mod prod;
//   type Output = Rc<Expr>;
//   fn mul(&self, rhs: Expr) -> Self::Output {
//     match Rc::<Expr>::get_mut(self) {
//         Some(s) => {
//           todo!("AA")
//         },
//         None => {
//           match (&*self, rhs) {
//             (Expr::Val(vs), Expr::Val(vr)) => {
//               let m = Rc::<Expr>::get_mut(self);
//               Rc::new(Expr::Val(vs*vr))
//             },
//             _ => {
//               todo!("Implement multiplication for other Rc types")
//             }
//         }
//         }
//     }
      
//   }
// }

// impl PartialEq for Expr {
//   fn eq(&self, other: &Self) -> bool {
//       false
//   }
// }

// impl Mul for Expr {
//   type Output = Self;
//   fn mul(&self, rhs: Self) -> Self::Output {
//       match self {
//         Expr::Frac( f ) => {
//           match rhs {
//               Expr::Frac(rhs_f) => {
//                 Expr::Frac(f*rhs_f)
//               },
//               Expr::Sum { terms } => {
//                 let v = Vec::new();
//                 for (coeff, factor) in terms {
//                   v.push((f*coeff,factor));
//                 }
//                 Expr::Sum { terms: v }
//               },
//               // All others: Put it inside a sum with one term
//               rhs => {
//                 Expr::Sum { terms: vec![(f, Box::new(rhs))] }
//               }
//           }
//         },
//         Expr::Const { ch, f64 } => {
//           match rhs {
//               Expr::Const { ch, f64 } => {
//                 if self == rhs {
//                   Expr::Prod { factors: vec![(Box::new(self),F::from(2))] }
//                 } else {
//                   Expr::Prod { factors: vec![(Box::new(self),F::from(1)),(Box::new(rhs),F::from(1))] }
//                 }
//               },
//               Expr::Sum { terms } => {
//                 let v = Vec::new()
//                 for (c,t) in terms {
//                   v.push((c, Box::new(*t*self)))
//                 }
//                 Expr::Sum { terms: v }
//               },
//               Expr::Prod { factors } => {
//                 match factors.iter().position(|(e,p)| *e == self) {
//                   Some(p) => {
//                     let mut f = &factors[p];
//                     (*f).1 = (*f).1 + F::from(1);
//                     Expr::Prod { factors: factors }
//                   },
//                   None => {
//                     factors.push((Box::new(self),F::from(1)));
//                     Expr::Prod { factors: factors }
//                   }
//                 }
//               },
//               _ => {
//                 Expr::Prod { factors: vec![
//                   (Box::new(rhs),F::from(1)),
//                   (Box::new(self), F::from(1)),
//                 ]}
//               }
//           }
//         },
//         // We are a sum
//         // Only special case is if the other is a sum, then we may be able to simplify
//         // Otherwise, we just distribute the other over ourselves
//         Expr::Sum { terms } => {
//           match rhs {
//               Expr::Frac(f) => {
//                 rhs * self
//               },
//               Expr::Const { ch, f64 } => {
//                 rhs * self
//               }

//               Expr::Sum { terms: rhs_terms } => {
//                 for t in terms {
//                   for rhs_t in rhs_terms {

//                   }
//                 }
//               }
//               _ => {
//                 self
//               }
//           }
//           let v = Vec::new();
//           for (c, e) in terms {
//             v.push((c, Box::new((*e)*self)));
//           }
//           Expr::Sum { terms: v }
//         },
//         Expr::Prod { factors } => {

//         },
//         Expr::Sqrt { v } => {

//         },
//         Expr::Cos { v } => {

//         },
//         Expr::Sin { v } => {

//         }
//       }
//   }
// }

// // Leaf nodes of the structure:
// // Sqrt(Fraction)
// // Const
// // 
// // Sqrt(F) - special multiplication with Sqrt(F)
// // Also special multiplication for 
// // I think we don't really have to care about leaf nodes.
// pub struct TSum {

// }

// pub enum FMul {
//   frac(F),

// }

// /// Square root
// /// 
// #[derive(Debug, PartialEq)]
// pub struct Sqrt<T> {
//   v: Box<T>
// }

// /// Composite factor
// /// 
// pub struct CompF<T> {
//   /// Coefficient
//   c: F,
//   s: Sqrt<T>,
//   /// Factors
//   fs: Vec<Const>,
// }


// #[test]
// fn sqrt_mul() {
//   let r = Sqrt{v:F::from(5)};
//   let q = Sqrt{v:F::from(2)};
//   assert_eq!(r*r, F::from(5));
//   assert_eq!(r*q, Sqrt{v:F::from(10)})
// }

// // impl<T> Mul for Sqrt<T> {
// //   fn mul(&self, rhs: Self) {

// //   }
// // }

// /// Something that cannot be further evaluated
// /// e.g. 'π', √2
// pub enum Atom {
//   F(F),
//   C(Const),
//   Sqrt(Sqrt<F>),
//   CompF(CompF<F>),
//   // Cos(Cos),
//   // Sin(Sin),
// }

// /// A multiplication of constants or functions
// /// Generally cannot be evaluated further
// /// e.g. π²√2
// // pub struct Mul {
// //   vals: Vec<(Atom,u32)>
// // }

// pub struct Fn {
//   ch: char
// }

// /// A composite value
// /// e.g. 1/2+1/2√5
// pub enum Val {
//   C(Const),
//   F(F),
//   // Ex(Expression),
// }

// // pub struct Term {
// //   c: F,

// // }

// // lazy_static!{
// //   static ref EXACT_CONSTS: HashMap<char, Value<'static>> = {
// //     let mut m = HashMap::new();
// //     let o = Value::Const(CFactor{name: 'o', f64: 0.0});
// //     let x = Value::Const(CFactor{name: 'x', f64: 1.0});
// //     let u = Value::Const(CFactor{name: 'u', f64: 2.0});

// //     let q = Value::SimpleSqrt(Sqrt { s_val: Frac::from(2) });
// //     let h = Value::SimpleSqrt(Sqrt { s_val: Frac::from(3) });
// //     let r = Value::SimpleSqrt(Sqrt { s_val: Frac::from(5) });
// //     let vector = vec![o,x,u,q,h,r];
// //     for val in vector {
// //       m.insert(val.name(), val);
// //     }
// //     let f = Value::Ex(Expression{name: 'f', terms: vec!
// //         [
// //           Term{coeff: Frac::new(1u8,2u8), factor: &m[&'x']},
// //           Term{coeff: Frac::new(1u8,2u8), factor: &m[&'r']},
// //         ]}
// //       );
// //     // m.insert(f.name(), f);
// //     m
// //   };
// // }

// // macro_rules! impl_trait_for_enum {
// //   ($trait_name:ident for $enum_name:ident {
// //       $( $variant:ident($type:ty), )*
// //   } => ($method:ident : $method_type:ty) ) => {
// //       impl $trait_name<$method_type> for $enum_name {
// //           fn $method(&self) -> $method_type {
// //               match self {
// //                   $(
// //                       $enum_name::$variant(inner) => inner.$method(),
// //                   )*
// //               }
// //           }
// //       }
// //   };
// // }


// // / Constant factor

// // #[derive(Debug)]
// // pub struct CFactor {
// //   name: char,
// //   f64: f64,
// // }

// // impl PartialEq for CFactor {
// //   fn eq(&self, other: &Self) -> bool {
// //       self.f64 == other.f64
// //   }
// // }

// // impl TryFrom<&CFactor> for f64 {
// //   // Cannot error
// //   type Error = ();
// //   fn try_from(value: &CFactor) -> Result<Self, Self::Error> {
// //       Ok(value.f64)
// //   }
// // }

// // impl Name for CFactor {
// //   fn name(&self) -> char{
// //     self.name
// //   }
// // }

// // /// Square root
// // /// e.g. √2, √(1+√2)
// // #[derive(Debug, PartialEq, Eq)]
// // pub struct Sqrt<T> where T: TryInto<f64>{
// //   s_val: T,
// // }

// // impl<T: Clone> TryFrom<&Sqrt<T>> for f64 where f64: TryFrom<T>{
// //   type Error = <f64 as TryFrom<T>>::Error;
// //   fn try_from(value: &Sqrt<T>) -> Result<Self, Self::Error> {
// //       match f64::try_from(value.s_val.clone()) {
// //         Ok(s_64) => {
// //           Ok(s_64.sqrt())
// //         }, Err(e) => {
// //           Err(e)
// //         }
// //       }
// //   }
// // }

// // impl<T> Name for Sqrt<T> where T: TryInto<f64>{
// //   fn name(&self) -> char {
// //       return '√'
// //   }
// // }


// // // impl Name for Value<'_> {
// // //   fn name(&self) -> char {
// // //       match self {

// // //       }
// // //   }
// // // }

// // /// A value that
// // #[derive(Debug, PartialEq)]
// // pub enum Value<'a> {
// //   Term(Term<'a>),
// //   Const(CFactor),
// //   Ex(Expression<'a>),
// //   SimpleSqrt(Sqrt<Frac>),
// //   // CompSqrt(Sqrt<&'a Expression<'a>>),
// //   CompF(Vec<&'a Value<'a>>),
// // }

// // // impl_trait_for_enum!(Name for Value { Const(CFactor), Ex(Sqrt), Term(Term) => name : char });
// // // impl Name for Value<'_> {
// // //   fn name(&self) -> char {
// // //       match self {
// // //         Value::Const(c) => {
// // //           c.name()
// // //         },
// // //         Value::Ex(e) => {
// // //           e.name()
// // //         },
// // //         Value::Term(t) => {
// // //           t.factor.name()
// // //         }
// // //         Value::CompF(c) => {
// // //           todo!("naming for composite factor")
// // //         },
// // //         Value::SimpleSqrt(s) => {
// // //           todo!("naming for simple square roots")
// // //         }
// // //       }
// // //   }
// // // }

// // #[derive(Debug, PartialEq)]
// // pub struct Term<'b> {
// //   coeff: Frac,
// //   factor: &'b Value<'b>
// // }

// // #[derive(Default, Debug, PartialEq)]
// // pub struct Expression<'a> {
// //   name: char,
// //   terms: Vec<Term<'a>>
// // }

// // impl Name for Expression<'_> {
// //     fn name(&self) -> char {
// //         self.name
// //     }
// // }

// // impl Name for CFactor {
// //   fn name(&self) -> char {
// //       self.val
// //   }
// // }

// // impl Description for CFactor {
// //   fn description(&self) -> &'static str {
// //       "CFactor"
// //   }
// // }

// // // #[derive(Debug)]
// // // struct Sqrt {
// // //   // ... your Sqrt implementation ...
// // // }

// // impl Name for Sqrt {
// //   fn name(&self) -> char {
// //       // ... your name logic for Sqrt ...
// //       'S'
// //   }
// // }

// // impl<T> Description for Sqrt<T> {
// //   fn description(&self) -> &'static str {
// //       "Sqrt"
// //   }
// // }

// // // #[derive(Debug)]
// // // struct Term {
// // //   factor: CFactor,
// // // }

// // impl Name for Term<'_> {
// //   fn name(&self) -> char {
// //       self.factor.name()
// //   }
// // }

// // impl Description for Term<'_> {
// //   fn description(&self) -> &'static str {
// //       "Term"
// //   }
// // }

// // // #[derive(Debug)]
// // // enum Value<'a> {
// // //   Const(CFactor),
// // //   Ex(Sqrt),
// // //   Term(Term),
// // // }

// // impl_trait_for_enum!(Name for Value, name => char);
// // // impl_trait_for_enum!(Description for Value, description => &'static str);

// // // fn main() {
// // //   let value = Value::Term(Term { factor: CFactor { val: 'T' } });

// // //   println!("Name: {}", value.name());
// // //   println!("Description: {}", value.description());
// // // }


// // #[test]
// // fn c_factor(){
// //   let pi = Value::Const(CFactor {
// //     name: 'π',
// //     f64: std::f64::consts::PI
// //   });
// //   let one = Value::Const(CFactor {
// //     name: 'x',
// //     f64: 1.0,
// //   });
// //   assert_eq!(pi.name(), 'π');
// //   assert_eq!(f64::try_from(&pi), Ok(std::f64::consts::PI));
// //   assert_eq!(pi + pi, Term{coeff: Frac::from(2), factor: &pi});
// //   assert_eq!(pi + one, Expression{terms: vec![
// //     Term{coeff: Frac::from(1), factor: &one},
// //     Term{coeff: Frac::from(1), factor: &pi},
// //   ]})
// //   // multiplication: not defined!
// //   // assert_eq!(pi * pi, )
// //   // assert_eq!(one * pi, pi);
// //   // 
// // }