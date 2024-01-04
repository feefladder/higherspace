use std::ops::Add;
use num::{One, Zero};

use super::*;

impl Add for Expr {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    // additive identity
    if self.is_zero() {return rhs;}
    if rhs.is_zero() {return self;}

    match (&self, &rhs) {
      (Expr::Val(a), Expr::Val(b)) => {
        let f_a: F = **a; let f_b: F = **b;
        Expr::val_frac(f_a + f_b)
      }
      (Expr::Val(v), Expr::Sum(s)) => {
        let mut v_s: Vec<(F, Expr)> = s.terms.clone();
        // check if sum already has our val
        match v_s.iter_mut().find(|(_,e)| e.is_one()) {
          Some((f_mut,_)) => {
            *f_mut += **v;
          },
          None => {
            v_s.push((**v,Expr::one()));
          }
        }
        v_s.retain(|(c,_)| c != &F::zero());
        Expr::from(v_s)
      }
      (_,Expr::Val(_))=>{rhs+self}
      (Expr::Sum(a),Expr::Sum(b)) => {
        let mut v_a: Vec<(F, Expr)> = a.terms.clone();
        let v_b: Vec<(F, Expr)> = b.terms.clone();
        for (coeff,fact) in v_b {
          // check if sum already has our val
          match v_a.iter_mut().find(|(_,e)| e == &fact) {
            Some((f_mut,_)) => {
              *f_mut += coeff;
            },
            None => {
              v_a.push((coeff,Expr::one()));
            }
          }
        }
        v_a.retain(|(c,_)| c != &F::zero());
        Expr::from(v_a)
      },
      (Expr::Sum(s), _) => {
        let mut v_s: Vec<(F, Expr)> = s.terms.clone();
        // check if sum already has our val
        match v_s.iter_mut().find(|(_,e)| e == &rhs) {
          Some((f_mut,_)) => {
            *f_mut += 1;
          },
          None => {
            v_s.push((F::one(),rhs));
          }
        }
        v_s.retain(|(c,_)| c != &F::zero());
        Expr::from(v_s)
      },
      (_, Expr::Sum(_)) => {
        rhs + self
      },
      (_,_) => {
        Expr::from(vec![
          (F::one(), self.clone()),
          (F::one(), rhs.clone()),
        ])
      }
    }
  }
}

#[cfg(test)]
mod test_e_rc_add {
  use super::*;
  use crate::io::ParseDisplay;

  #[test]
  fn test_add_zero() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("ξ1", "ξ0", "ξ1"),
      ("ξ1","ξ-1", "ξ0"),
      ("π", "ξ0", "π"),
      ("π","Σ(-1,π)", "ξ0"),
      ("Σ[(1,ξ1),(1,π)]","ξ-1","π"),
      ("Σ[(1,ξ1),(1,π)]","Σ(-1,π)","ξ1"),
    ];
    // assert_eq!(Ok(Expr::from(Vec::<(F,Expr)>::new())),Expr::parse_display("".to_string()));
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() + e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b + e_a) , e_ans);
    }
  }

  #[test]
  fn test_val_plus_val() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("ξ1", "ξ1", "ξ2"),
      ("ξ2", "ξ2", "ξ4"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() + e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b + e_a) , e_ans);
    }
  }


  #[test]
  fn test_val_plus_sum() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("ξ2", "Σ(3,π)", "Σ[(3,π),(2,ξ1)]"),
      ("ξ2", "Σ[(3,π),(2,ξ1)]", "Σ[(3,π),(4,ξ1)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() + e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b + e_a) , e_ans);
    }
  }
}