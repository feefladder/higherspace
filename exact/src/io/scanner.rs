/// Additional QOL functions and member variables for lyn::Scanner struct
/// Implemented deref trait as suggested by ChatGPT
use std::{
  ops::{
    Deref,
    DerefMut,
  },
  collections::{HashMap, hash_map::RandomState}
};

/// Parser for parsing display-type strings
/// e.g. 
use lyn::{Scanner as Sc, Error};

use crate::{
  structs::Const,
  io::traits::Char,
};

pub struct Scanner{
  inner: Sc,
  c_map: HashMap<char, Const, RandomState>
}

// /// Wrap methods for functions that are actually functions of an inner field
// /// 
// /// ```rust
// /// struct with_inner {
// /// i_name: SomeStruct
// /// }
// /// ```
// /// where SomeStruct implements
// /// ```rust
// /// impl SomeStruct {
// ///   pub fn meth1(&self) -> Type1 {
// ///     Type1::new()
// ///   }
// ///   pub fn m_args(&self, arg1: T2) -> T3 {
// ///     T3::new()
// ///   }
// /// }
// /// ```
// /// use like:
// /// ```rust
// /// wrap_meth!(
// ///   <i_name>,
// ///   meth1 -> Type1,
// ///   m_args(arg1: T2) -> T3,
// /// )
// /// ```
// /// where methods without arguments precede methods with args.
// macro_rules! wrap_meth {

//   // Base case: wrap a single method with a specified return type
//   ($method:ident -> $return_type:ty) => {
//     pub fn $method(&self) -> $return_type {
//         self.inner.$method()
//     }
// };

// // Recursive case: wrap multiple methods with specified return types
// ($($method:ident -> $return_type:ty),*$(,)*) => {
//     $(wrap_meth!($method -> $return_type);)*
// };
//   // (
//   //   // <$field:ident>,
//   //   // $(
//   //     m:ident -> r:ty
//   //   // ),*
//   // ) => {
//   //   // $(
//   //     pub fn $m(&self) -> $r {
//   //       self.inner.$m()
//   //     }
//   //   // )*
//   // };

//   // Base case: wrap a single method with a specified return type
//   // (<$field:ident>,$method:ident -> $return_type:ty) => {
      
//   // };

//   // Case with optional arguments
//   // m        opt_args                   -> r_type
//   // (
//   //   <$field:ident>,
//   //   $m_args:ident($($arg:ident: $t:ty),*) -> $r_type:ty$(,)*
//   // ) => {
//   //     pub fn $m_args(&self, $($arg: $t),*) -> $r_type {
//   //         self.$field.$m_args($($arg),*)
//   //     }
//   // };

//   // // Recursive case: wrap multiple methods with specified return types
//   // (
//   //   <$field:ident>$(,)*
//   //   $(
//   //     $($meth:ident -> $r_meth_type:ty)*$(,)*
//   //     |
//   //     $(m_args:ident($($arg:ident: $a_t:ty),*) -> $r_at_t:ty)*$(,)*
//   //   )*
//   // ) => {
//   //     $(wrap_meth!(<$field>$meth -> $r_meth_type);)*
//   //     |
//   //     $(wrap!(<$field>$m_args($($arg: $a_t),*) -> $r_a_t);)*
//   // };


//   // Recursive case with optional arguments
//   (<$field:ident>$($m:ident($($arg:ident: $a_type:ty),*) -> $r_type:ty),* $(,)*) => {
//       $(wrap_meth!(<$field>,$m($arg: $a_type));)*
//   };

//   // Recursive without following with args
// }


impl Scanner {
  pub fn new(string: &str) -> Self {
    Scanner { inner: Sc::new(string), c_map: Scanner::c_map() }
  }

  pub fn c_map() -> HashMap<char, Const, RandomState> {
    let mut m = HashMap::new();
    let v = vec![
      Const {
        ch: 'π',
        f64: std::f64::consts::PI
      },
      Const {
        ch: 'e',
        f64: std::f64::consts::E,
      }
    ];
    for c in v {
      m.insert(c.ch(), c);
    }
    m
  }

  pub fn has(&self, c: &char) -> bool {
    self.c_map.contains_key(c)
  }

  pub fn get_const(&self, c: &char) -> Const {
    self.c_map[c]
  }

  pub fn err_or_eq(&mut self, c: &char) -> Result<(), Error>{
    if !self.take(&c) {
      eprintln!("Did not find {} at position: {}, found: {}", c, self.cursor(), self.peek().unwrap_or(&'🕘'));
      Err(Error::Character(self.cursor()))
    } else {
      Ok(())
    }
  }
  // wrap_meth!(
  //   // <inner>,
  //   cursor -> usize,
  //   peek -> Option<&char>,
  //   is_done -> bool,
  //   pop -> Option<&char>,
  //   // scan<T>,
  //   // transform<T>,
  // );

  // wrap_meth!(
  //   <inner>,
  //   take(target: &char) -> bool,
  // );

  // wrap!();
  // pub fn take(&self, target: &char) -> bool{
  //   self.inner.take(target)
  // } 
}

impl Deref for Scanner {
  type Target = Sc;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

impl DerefMut for Scanner {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

// TODO: tests
