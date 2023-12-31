
// use crate::structs::{Sqrt};

/// Name of a Value. Should return char
/// e.g. Sqrt<T> -> √
/// 
trait Char {
  fn ch(&self) -> char;
}

/// Representation. Should return a string that results in the same valeu
/// e.g. assert_eq!(exact::from(some_exact::repr()), some_exact)
trait Repr {
    fn repr(&self) -> String;
}
