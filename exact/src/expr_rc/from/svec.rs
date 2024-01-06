use std::rc::Rc;
use crate::{expr_rc::{SVec, Expr}, structs::Sum};

use super::FromRaw;

impl FromRaw<SVec> for Expr {
  fn from_raw(terms: SVec) -> Self {
      Expr::Sum(Rc::new(Sum{ terms}))
  }
}