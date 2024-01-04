use lyn::Error;

use crate::{expr_rc::Expr, structs::Const};

pub trait ParseDisplay {
  fn parse_display(input: String) -> Result<Expr, Error>;
}

/// Name of an Expr variant. Should return char
/// Val  -> ξ
/// Const -> <char of const>
/// Sum  -> Σ
/// Prod -> Π
/// Sqrt -> √
/// For sin, cos, tan, they are a clock based on this triangle:
/// Cos   Sin   Tan
/// ____
/// |  /  /|    /|
/// |-/  / |   / |
/// |/  /\_|  /\_|
/// Cos Sin Tan
/// |/  /|  _|
/// 🕑  🕢  🕘
/// Sin  -> 🕢
/// Cos  -> 🕑
/// Tan  -> 🕘
pub trait Char {
  fn ch(&self) -> char;
}

/// Const -> <char of const>
// impl Char for Const {
//   fn ch(&self) -> char {
//     self.ch
//   }
// }

/// Name of an Expr variant. Should return char
/// Val  -> ξ
/// Const -> <char of const>
/// Sum  -> Σ
/// Prod -> Π
/// Sqrt -> √
/// For sin, cos, tan, they are a clock based on this triangle:
/// Cos   Sin   Tan
/// ____
/// |  /  /|    /|
/// |-/  / |   / |
/// |/  /\_|  /\_|
/// Cos Sin Tan
/// |/  /|  _|
/// 🕑  🕢  🕘
/// Sin  -> 🕢
/// Cos  -> 🕑
/// Tan  -> 🕘
impl Char for Expr {
    fn ch(&self) -> char {
        match self {
          Expr::Val(_) => 'ξ',
          Expr::Const(c) => c.ch(),
          Expr::Sum(_) => 'Σ',
          Expr::Prod(_) => 'Π',
          Expr::Sqrt(_) => '√',
        }
    }
}

#[cfg(test)]
mod test_io_traits{
  use super::*;
  use crate::expr_rc::Expr;

  #[test]
  fn test_expr_char() {
    assert_eq!(Expr::val_i(2).ch(),'ξ');
    assert_eq!(Expr::c_pi().ch(),'π');
    assert_eq!(Expr::sum_i_pi(2).ch(),'Σ');
    assert_eq!(Expr::prod_pi_i(1).ch(),'Π');
    assert_eq!(Expr::sqrt_i(5).ch(),'√');
  }
}