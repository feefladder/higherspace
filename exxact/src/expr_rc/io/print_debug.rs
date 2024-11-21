use crate::{
  expr_rc::Expr
  // io::print_display::Display
};

use std::fmt::{Debug};

impl Debug for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self)
  }
}