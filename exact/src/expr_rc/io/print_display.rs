use std::fmt::Display;


// use super::*;
use crate::{
  io_traits::Char,
  expr_rc::{
    Expr, GetExpr,
    io::{scanner::Scanner},
  }
};



/// Representation. Should return a string that results in the same value
/// e.g. assert_eq!(exact::from(format!("{}",some_exact), some_exact)
/// For example:
/// 1/2+1/2√(5/2) -> Σ((1/2,ξ1),(1/2,√5/2))
/// Note that currently constants loose precision e.g.
/// Const{ch: 'π', f64: std::consts::f64::PI} -> π
/// Except that bla
impl Display for Expr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f,"{}",self.ch())?;
    match &self {
      // 5 -> ξ5
      &Expr::Val(v) => {
        write!(f,"{}",v)
      },
      // π -> π
      &Expr::Const(c) => {
        if Scanner::c_map().contains_key(&c.ch()) {
          write!(f, "")
        } else {
          write!(f, "({:?})", c.f64)
        }
      },
      // 2+pi -> Σ[(2,ξ1),(1,π)]
      // 2'pi' -> Σ(2,'pi')
      &Expr::Sum(s) => {
        if s.terms.len() == 1 {
          let (coeff,fact) = &s.terms[0];
          write!(f,"({},{})",coeff,fact)
        } else {
          let (c0, f0) = &s.terms[0];
          // opening bracket and first term for comma stuff
          write!(f,"[({},{})", c0, f0)?;
          for (coeff,fact) in &s.terms[1..] {
            write!(f,",({},{})",coeff,fact)?;
          }
          write!(f,"]")
        }
      },
      // pi² -> Π((π,2))
      &Expr::Prod(p) => {
        if p.factors.len() == 1 {
          let (fact, exp) = &p.factors.first().unwrap();
          write!(f,"({},{})",fact,exp)
        } else {
          let (f0, e0) = &p.factors[0];
          // opening bracket and first term for comma stuff
          write!(f,"[({},{})", f0, e0)?;
          for (fact, exp) in &p.factors[1..] {
            write!(f,",({},{})",fact, exp)?;
          }
          write!(f,"]")
        }
      },
      // √'pi' -> √'pi'
      // √5 -/-> √ξ5
      //  \-> √5
      _ => {
        let expr = self.get_expr().unwrap();
        match expr {
            Expr::Val(v) => {
              write!(f,"{}",v)
            }
            _ => {
              write!(f, "{}",expr)
            }
        }

      }
    }
  }
}

#[cfg(test)]
mod printd_tests{
  use super::*;



  mod display_tests {
    use super::*;
    use crate::F;
    #[test]
    fn test_expr_display() {
      // 1/2+1/2√(5/2) -> Σ((1/2,ξ1),(1/2,√5/2))
      assert_eq!(format!("{}",Expr::val_i(2)),"ξ2");
      assert_eq!(format!("{}",Expr::val_frac(F::new(5u8,2u8))),"ξ5/2");
      assert_eq!(format!("{}",Expr::c_pi()),"π");
      assert_eq!(format!("{}",Expr::from(('φ',1.618))),"φ(1.618)");
      assert_eq!(format!("{}",Expr::sqrt_i(5)),"√5");
      assert_eq!(format!("{}",Expr::sqrt_frac(F::new(5u8,2u8))),"√5/2");
      assert_eq!(format!("{}",Expr::sqrt_expr(Expr::c_pi())),"√π");
      assert_eq!(format!("{}",Expr::sqrt_expr(Expr::sum_i_pi(2))),"√Σ(2,π)");
      assert_eq!(format!("{}",Expr::sum_i_pi(2)),"Σ(2,π)");
      assert_eq!(format!("{}",Expr::sum_i_plus_pi(2)),"Σ[(2,ξ1),(1,π)]");
      assert_eq!(format!("{}",Expr::sum_phi()),"Σ[(1/2,ξ1),(1/2,√5)]");
      assert_eq!(format!("{}",Expr::prod_pi_i(2)),"Π(π,2)");
      assert_eq!(format!("{}",Expr::prod_pi_times_sqrt_i(5)),"Π[(π,1),(√5,1)]");
    }
  }
}