use fraction::{Fraction, Sign, ToPrimitive, GenericFraction};
use std::collections::HashMap;
type Frac = GenericFraction<u32>;

lazy_static!{
  static ref EXACT_CONSTS: HashMap<char, Value<'static>> = {
    let mut m = HashMap::new();
    let o = Value::Const(CFactor{name: 'o', f64: 0.0});
    let x = Value::Const(CFactor{name: 'x', f64: 1.0});
    let u = Value::Const(CFactor{name: 'u', f64: 2.0});

    let q = Value::SimpleSqrt(Sqrt { s_val: Frac::from(2) });
    let h = Value::SimpleSqrt(Sqrt { s_val: Frac::from(3) });
    let r = Value::SimpleSqrt(Sqrt { s_val: Frac::from(5) });
    let vector = vec![o,x,u,q,h,r];
    for val in vector {
      m.insert(val.name(), val);
    }
    let f = Value::Ex(Expression{name: 'f', terms: vec!
        [
          Term{coeff: Frac::new(1u8,2u8), factor: &m[&'x']},
          Term{coeff: Frac::new(1u8,2u8), factor: &m[&'r']},
        ]}
      );
    // m.insert(f.name(), f);
    m
  };
}

// macro_rules! impl_trait_for_enum {
//   ($trait_name:ident for $enum_name:ident {
//       $( $variant:ident($type:ty), )*
//   } => ($method:ident : $method_type:ty) ) => {
//       impl $trait_name<$method_type> for $enum_name {
//           fn $method(&self) -> $method_type {
//               match self {
//                   $(
//                       $enum_name::$variant(inner) => inner.$method(),
//                   )*
//               }
//           }
//       }
//   };
// }

/// Name of a Value. Should return char
/// e.g. Sqrt<T> -> √
/// 
trait Name {
  fn name(&self) -> char;
}

/// Constant factor
/// e.g. 'π' = std::f64::consts::PI
#[derive(Debug)]
pub struct CFactor {
  name: char,
  f64: f64,
}

impl PartialEq for CFactor {
  fn eq(&self, other: &Self) -> bool {
      self.f64 == other.f64
  }
}

impl TryFrom<&CFactor> for f64 {
  // Cannot error
  type Error = ();
  fn try_from(value: &CFactor) -> Result<Self, Self::Error> {
      Ok(value.f64)
  }
}

impl Name for CFactor {
  fn name(&self) -> char{
    self.name
  }
}

/// Square root
/// e.g. √2, √(1+√2)
#[derive(Debug, PartialEq, Eq)]
pub struct Sqrt<T> where T: TryInto<f64>{
  s_val: T,
}

impl<T: Clone> TryFrom<&Sqrt<T>> for f64 where f64: TryFrom<T>{
  type Error = <f64 as TryFrom<T>>::Error;
  fn try_from(value: &Sqrt<T>) -> Result<Self, Self::Error> {
      match f64::try_from(value.s_val.clone()) {
        Ok(s_64) => {
          Ok(s_64.sqrt())
        }, Err(e) => {
          Err(e)
        }
      }
  }
}

impl<T> Name for Sqrt<T> where T: TryInto<f64>{
  fn name(&self) -> char {
      return '√'
  }
}


// impl Name for Value<'_> {
//   fn name(&self) -> char {
//       match self {

//       }
//   }
// }

/// A value that
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
  Term(Term<'a>),
  Const(CFactor),
  Ex(Expression<'a>),
  SimpleSqrt(Sqrt<Frac>),
  // CompSqrt(Sqrt<&'a Expression<'a>>),
  CompF(Vec<&'a Value<'a>>),
}

// impl_trait_for_enum!(Name for Value { Const(CFactor), Ex(Sqrt), Term(Term) => name : char });
// impl Name for Value<'_> {
//   fn name(&self) -> char {
//       match self {
//         Value::Const(c) => {
//           c.name()
//         },
//         Value::Ex(e) => {
//           e.name()
//         },
//         Value::Term(t) => {
//           t.factor.name()
//         }
//         Value::CompF(c) => {
//           todo!("naming for composite factor")
//         },
//         Value::SimpleSqrt(s) => {
//           todo!("naming for simple square roots")
//         }
//       }
//   }
// }

#[derive(Debug, PartialEq)]
pub struct Term<'b> {
  coeff: Frac,
  factor: &'b Value<'b>
}

#[derive(Default, Debug, PartialEq)]
pub struct Expression<'a> {
  name: char,
  terms: Vec<Term<'a>>
}

impl Name for Expression<'_> {
    fn name(&self) -> char {
        self.name
    }
}

macro_rules! impl_trait_for_enum {
  ($trait_name:ident for $enum_name:ident, $method:ident => $method_type:ty) => {
      impl $trait_name<$method_type> for $enum_name<'_> {
          fn $method(&self) -> $method_type {
              match self {
                  $enum_name::$variant(inner) => inner.$method(),
                  // Add more variants as needed
              }
          }
      }
  };
}

// trait Name {
//   fn name(&self) -> char;
// }

trait Description {
  fn description(&self) -> &'static str;
}

// #[derive(Debug)]
// struct CFactor {
//   val: char,
// }

impl Name for CFactor {
  fn name(&self) -> char {
      self.val
  }
}

impl Description for CFactor {
  fn description(&self) -> &'static str {
      "CFactor"
  }
}

// #[derive(Debug)]
// struct Sqrt {
//   // ... your Sqrt implementation ...
// }

impl Name for Sqrt {
  fn name(&self) -> char {
      // ... your name logic for Sqrt ...
      'S'
  }
}

impl<T> Description for Sqrt<T> {
  fn description(&self) -> &'static str {
      "Sqrt"
  }
}

// #[derive(Debug)]
// struct Term {
//   factor: CFactor,
// }

impl Name for Term<'_> {
  fn name(&self) -> char {
      self.factor.name()
  }
}

impl Description for Term<'_> {
  fn description(&self) -> &'static str {
      "Term"
  }
}

// #[derive(Debug)]
// enum Value<'a> {
//   Const(CFactor),
//   Ex(Sqrt),
//   Term(Term),
// }

impl_trait_for_enum!(Name for Value, name => char);
// impl_trait_for_enum!(Description for Value, description => &'static str);

// fn main() {
//   let value = Value::Term(Term { factor: CFactor { val: 'T' } });

//   println!("Name: {}", value.name());
//   println!("Description: {}", value.description());
// }


#[test]
fn c_factor(){
  let pi = Value::Const(CFactor {
    name: 'π',
    f64: std::f64::consts::PI
  });
  let one = Value::Const(CFactor {
    name: 'x',
    f64: 1.0,
  });
  assert_eq!(pi.name(), 'π');
  assert_eq!(f64::try_from(&pi), Ok(std::f64::consts::PI));
  assert_eq!(pi + pi, Term{coeff: Frac::from(2), factor: &pi});
  assert_eq!(pi + one, Expression{terms: vec![
    Term{coeff: Frac::from(1), factor: &one},
    Term{coeff: Frac::from(1), factor: &pi},
  ]})
  // multiplication: not defined!
  // assert_eq!(pi * pi, )
  // assert_eq!(one * pi, pi);
  // 
}