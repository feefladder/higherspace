use std::ops::Mul;
use num::{One, Zero};
use num_traits::Inv;
use super::*;

impl Mul for Expr {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    // Multiplicative identity
    if self.is_one() {return rhs.clone();}
    if rhs.is_one() {return self.clone();}
    // Additive identity
    if self.is_zero() || rhs.is_zero() {return Expr::zero();}
    // Multiplicative inverse
    if self.clone().inv() == rhs {return Expr::one();}

    match (&self,&rhs) {
      // 
      // Value
      // 
      (Expr::Val(a), Expr::Val(b)) => {
        let f_a: F = **a; let f_b: F = **b;
        Expr::val_frac(f_a*f_b)
      },
      (Expr::Val(v), Expr::Sum(s)) => {
        let frac: F = **v;
        let mut v: Vec<(F, Expr)> = s.terms.clone();
        for (f , _) in v.iter_mut() {
          *f *=frac;
        }
        Expr::from(v)
      },
      (Expr::Val(v), _) => {
        let f: F = **v;
        Expr::from(vec![(f,rhs)])
      },
      (_, Expr::Val(_)) => {
        rhs * self
      },
      //
      // Const
      //
      (Expr::Const(a), Expr::Const(b)) => {
        if a == b {
          Expr::from(vec![(self.clone(),F::from(2))])
        } else {
          Expr::from(vec![
            (self.clone(),F::one()),
            (rhs.clone(),F::one()),
          ])
        }
      },
      (Expr::Const(_),Expr::Sum(s)) => {
        // recursive
        let s_v: Vec<(F, Expr)> = s.terms.clone();
        let mut v_new = Vec::new();
        for (f , e) in s_v {
          v_new.push((f,self.clone()*e))
        }
        Expr::from(v_new)
      },
      (Expr::Const(_),Expr::Prod(p)) => {
        let mut p_v: Vec<(Expr, F)> = p.factors.clone();
        match p_v.iter_mut().find(|(e,_)| e == &self) {
          Some((_,exp)) => {
            *exp += 1;
          },
          None => {
            p_v.push((self.clone(),F::one()))
          }
        }
        Expr::from(p_v)
      },
      (Expr::Const(_),_) => {
        Expr::from(vec![
          (self,F::one()),
          (rhs, F::one()),
        ])
      }
      (_,Expr::Const(_)) => {
        rhs * self
      }
      // 
      // Sum
      // 
      // Σ(2,π)           * Σ(2,π)  = Σ(4,Π(π,2))"),
      // Σ[(1,ξ1),(1,√5)] * Σ(2,√5) = Σ[(2,√5),(20,ξ1)]
      // Σ[(1,ξ1),(1,√5)] ^2        = Σ[(2,√5),(6,ξ1)]
      (Expr::Sum(a),Expr::Sum(b)) => {
        let mut expr_new = Expr::zero();
        let v_a: &Vec<(F,Expr)> = &a.terms.clone();
        let v_b: &Vec<(F,Expr)> = &b.terms.clone();
        for (f_a, e_a) in v_a {
          for (f_b, e_b) in v_b {
            let f_ab = f_a * f_b;
            let e_ab = e_a.clone() * e_b.clone();

            expr_new = expr_new + Expr::from(vec![(f_ab, e_ab)])
          }
        }
        expr_new
      },
      (Expr::Sum(s), Expr::Prod(p)) => {
        todo!()
      },
      (Expr::Sum(sum), Expr::Sqrt(sqrt)) => {
        todo!()
      },
      (_,Expr::Sum(_)) => {
        rhs * self
      },
      // 
      // Prod
      // 
      (Expr::Prod(a), Expr::Prod(b)) => {
        let mut v_a: Vec<(Expr, F)> = a.factors.clone();
        let v_b: Vec<(Expr, F)> = b.factors.clone();
        // loop through v_b and add to v_a
        for (expr_b,exp_b) in v_b {
          match v_a.iter_mut().find(|(expr_a, _)| expr_a == &expr_b) {
              Some((_, exp_a)) => {
                *exp_a += exp_b;
              },
              None => {
                v_a.push((expr_b, exp_b));
              }
          }
        }
        Expr::from(v_a)
      },
      (Expr::Prod(p),Expr::Sqrt(s)) => {
        let mut v_p: Vec<(Expr, F)> = p.factors.clone();
        match v_p.iter_mut().find(|(expr_p,_)| matches!(expr_p, Expr::Sqrt(_))) {
          Some(i) => {

          },
          None => {

          }
        }
        todo!()
      },
      (_,Expr::Prod(_)) => {
        rhs * self
      },
      // 
      // Sqrt
      // 
      (Expr::Sqrt(a), Expr::Sqrt(b)) => {
        todo!()
      },
      (_,_) => {
        todo!("Implement multiplication for {},{}", self, rhs)
      }
    }
  }
}



// /// Multiplication for epressions.
// /// MAY mutate the input values, 
// impl Mul for Expr {
//   type Output = Self;
//   fn mul(mut self,mut rhs: Self) -> Self::Output {
//       match (&mut self, &mut rhs) {
//         (Expr::Val(vs), Expr::Val(vr)) => {
//           match (Rc::<F>::get_mut(vs), Rc::get_mut(vr)) {
//               (Some(v), _) => {
//                 // Mutate our own value and return self
//                 *v *= &**vr;
//                 return self;
//               },
//               (_, Some(v)) => {
//                 *v *= &**vs;
//                 rhs
//               },
//               (None, None) => {
//                 println!("Copying value!");
//                 Expr::Val(Rc::new(&**vs * &**vr));
//               }
//             }
//         },
//         (Expr::Val(vs), Expr::Const(cr)) => {
//           Expr::Sum(
//             Rc::new(
//               Sum{
//                 terms: vec![
//                   (
//                     **vs,
//                     Expr::Const(Rc::clone(cr));
//                   ),
//                 ]
//               }
//             );
//           );
//         },
//         (Expr::Const(_), Expr::Val(_)) => rhs*self,
//         (Expr::Val(vs), Expr::Sum(sum_r)) => {
//           match Rc::get_mut(sum_r) {
//             Some(s) => {
//               s.terms.iter_mut().for_each(|(c, e)| *c *= **vs);
//               rhs
//             },
//             None => {
//               Expr::Sum(
//                 Rc::new(
//                   Sum{
//                     terms: sum_r.terms.iter().map(|(c,e)| (*c*(**vs),*e));
//                     .collect();
//                   }
//                 );
//               );
//             }
//           }
//         },
//         (Expr::Sum(_),Expr::Val(_)) => rhs*self,
//         (Expr::Val(vs), _) => {
//           Expr::Sum(
//             Rc::new(
//               Sum{
//                 terms: vec![(**vs, rhs)]
//               }
//             );
//           );
//         },
//         (_, Expr::Val(_)) => rhs*self,
//         (Expr::Const(cs),Expr::Const(cr)) => {
//           if cs == cr {
//             Expr::Prod(Rc::new(
//               Prod {factors: vec![(&self, F::from(2))]}
//             ));
//           } else {
//             Expr::Prod(Rc::new(
//               Prod { factors: vec![(&self, F::from(1)),(rhs, F::from(1))] }
//             ));
//           }
//         },
//         (Expr::Const(cs), Expr::Sum(sr)) => {
//           match Rc::<Sum>::get_mut(sr) {
//               Some(sum_r) => {
//                 sum_r.terms.iter_mut().for_each(|(_, ex)| *ex = self * *ex);
//                 rhs
//               },
//               None => {
//                 Expr::Sum(
//                   Rc::new(
//                     Sum { 
//                       terms: sr.terms.iter().map(|(c, ex)| (*c, self* *ex)).collect();
//                      }
//                   );
//                 );
//               }
//           }
//         }
//         _ => {
//           todo!("implement multiplication for other types");
//         }
//       }
//   }
// }

#[cfg(test)]
mod test_e_rc_mul {
  use num::One;
  use super::*;
  use crate::io::traits::ParseDisplay;

  // for laziness 
  // DO NOT CHANGE 
  // (will break most tests bc results are parsed from string literals like "5".to_string())
  const i: u32 = 5;
  const f: (u32,u32) = (1,2);

  // macro_rules! test_v_times_b {
  //     ($f:ident $arg:ident) => {
  //       #[test]
  //       fn test_v_times_($f)() {
  //         assert_eq!(Expr::one() * Expr::$f($arg) , Expr::$f(i));
  //         assert_eq!(Expr::f($arg) * Expr::f($arg), Expr::f($arg*arg));
  //       }

  //       #[test]
  //       fn test_($f)_times_v() {
  //         assert_eq!(Expr::$f($arg) * Expr::one() , Expr::$f(i));
  //         assert_eq!(Expr::f($arg) * Expr::f($arg),  Expr::f($arg*arg));
  //       }
  //     };
  // }

  #[test]
  fn test_mul_inv() {
    let test_vec = vec![
      ("ξ1","ξ1"),
      ("ξ2","ξ1/2"),
      ("ξ-2","ξ-1/2"),
      ("π","Π(π,-1)"),
      ("Σ[(1,ξ1),(1,π)]","Π(Σ[(1,ξ1),(1,π)],-1)"),
      ("Π(π,2)","Π(π,-2)"),
      // super panic
      // ("Π[(π,1),(e,1)]","Π[(π,-1),(e,-1)]"),
      ("√2","Σ(1/2,√2)"),
    ];
    for (a,b) in test_vec {
      let e_a = Expr::try_from(a).unwrap();
      let e_b = Expr::try_from(b).unwrap();
      assert_eq!(e_a.clone() * e_b.clone(), Expr::one());
      assert_eq!(e_b * e_a, Expr::one());
    }
  }

  #[test]
  fn test_v_times_v() {
    assert_eq!(Expr::one() * Expr::one() , Expr::one());
    assert_eq!(Expr::one() * Expr::val_i(5) , Expr::val_i(5));
    assert_eq!(Expr::val_i(5) * Expr::val_i(5), Expr::val_i(25));
  }
  // test_v_times_b!(Expr::c_pi() )
  #[test]
  fn test_v_times_c() {
    assert_eq!(Expr::one() * Expr::c_pi() , Expr::c_pi());
    assert_eq!(Ok(Expr::one() * Expr::c_pi()) , Expr::parse_display("π".to_string()));
    assert_eq!(Ok(Expr::val_i(i) * Expr::c_pi()) , Expr::parse_display("Σ(5,π)".to_string()));
    assert_eq!(Ok(Expr::val_frac(F::from(f)) * Expr::c_pi()) , Expr::parse_display("Σ(1/2,π)".to_string()));
  }
  #[test]
  fn test_c_times_v() {
    assert_eq!(Expr::c_pi() * Expr::one() , Expr::c_pi());
    assert_eq!(Ok(Expr::c_pi() * Expr::one()) , Expr::parse_display("π".to_string()));
    assert_eq!(Ok(Expr::c_pi() * Expr::val_i(i)) , Expr::parse_display("Σ(5,π)".to_string()));
    assert_eq!(Ok(Expr::c_pi() * Expr::val_frac(F::from(f))) , Expr::parse_display("Σ(1/2,π)".to_string()));
  }
  #[test]
  fn test_v_times_sum() {
    assert_eq!(Expr::one() * Expr::sum_i_pi(i) , Expr::sum_i_pi(i));
    assert_eq!(Ok(Expr::one() * Expr::sum_i_pi(i) ), Expr::parse_display("Σ(5,π)".to_string()));
    assert_eq!(Ok(Expr::val_i(i) * Expr::sum_i_pi(i) ), Expr::parse_display("Σ(25,π)".to_string()));
    assert_eq!(Ok(Expr::val_i(i) * Expr::sum_i_plus_pi(i) ), Expr::parse_display("Σ[(25,ξ1),(5,π)]".to_string()));
  }
  #[test]
  fn test_v_times_prod() {
    assert_eq!(Expr::one() * Expr::prod_pi_i(i) , Expr::prod_pi_i(i));
    assert_eq!(Ok(Expr::one() * Expr::prod_pi_i(i) ), Expr::parse_display("Π(π,5)".to_string()));
    assert_eq!(Ok(Expr::val_i(i) * Expr::prod_pi_i(i) ), Expr::parse_display("Σ(5,Π(π,5))".to_string()));
    assert_eq!(Ok(Expr::val_frac(F::from(f)) * Expr::prod_pi_i(i) ), Expr::parse_display("Σ(1/2,Π(π,5))".to_string()));
  }
  #[test]
  fn test_v_times_sqrt() {
    assert_eq!(Expr::one() * Expr::sqrt_i(i) , Expr::sqrt_i(i));
    assert_eq!(Ok(Expr::one() * Expr::sqrt_i(i)) , Expr::parse_display("√5".to_string()));
    assert_eq!(Ok(Expr::val_i(i) * Expr::sqrt_i(i) ), Expr::parse_display("Σ(5,√5)".to_string()));
    assert_eq!(Ok(Expr::val_frac(F::from(f)) * Expr::sqrt_i(i) ), Expr::parse_display("Σ(1/2,√5)".to_string()));
  }

  #[test]
  fn test_c_times_c() {
    assert_eq!(Expr::c_pi() * Expr::c_pi() , Expr::prod_pi_i(2));
    assert_eq!(Ok(Expr::c_pi() * Expr::c_pi()), Expr::parse_display("Π(π,2)".to_string()));
    assert_eq!(Ok(Expr::c_pi() * Expr::c_e()), Expr::parse_display("Π[(π,1),(e,1)]".to_string()));
    assert_eq!(Ok(Expr::c_e() * Expr::c_pi()), Expr::parse_display("Π[(e,1),(π,1)]".to_string()));
    assert_eq!(Ok(Expr::c_e() * Expr::c_e()), Expr::parse_display("Π(e,2)".to_string()));
  }

  #[test]
  fn test_c_times_sum() {
    // single term
    // π*5π = 5π²
    assert_eq!(Ok(Expr::c_pi() * Expr::sum_i_pi(i)) , Expr::parse_display("Σ(5,Π(π,2))".to_string()));
    // e*5π = 5πe (?= 5eπ)
    assert_eq!(Ok(Expr::c_e() * Expr::sum_i_pi(i)) , Expr::parse_display("Σ(5,Π[(e,1),(π,1)])".to_string()));
    // π*(5+π) = 5π+π²
    assert_eq!(Ok(Expr::c_pi() * Expr::sum_i_plus_pi(i)),Expr::parse_display("Σ[(5,π),(1,Π(π,2))]".to_string()));
    assert_eq!(Ok(Expr::sum_i_plus_pi(i) * Expr::c_pi()),Expr::parse_display("Σ[(5,π),(1,Π(π,2))]".to_string()));
    
  }

  #[test]
  #[ignore = "equality TODO"]
  fn test_c_times_sum_fail() {
    // OOPS! this requires more loose equality rules or something
    // The first notation MAY be preferable
    // π*(1/2+1/2sqrt(5)) = 1/2π+1/2πsqrt(5)
    assert_eq!(Ok(Expr::c_pi() * Expr::sum_phi()),Expr::parse_display("Σ[(1/2,π),(1/2,Π[(π,1),(√5,1)])]".to_string()));
    assert_eq!(Ok(Expr::c_pi() * Expr::sum_phi()),Expr::parse_display("Π[(π,1),(Σ[(1/2,ξ1),(1/2,√5)],1)]".to_string()));
  }

  #[test]
  fn test_c_times_prod() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("π","Π(π,2)", "Π(π,3)"),
      ("e","Π(π,2)","Π[(π,2),(e,1)]"),
      ("π","Π[(√5,1),(e,1)]","Π[(√5,1),(e,1),(π,1)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  #[test]
  fn test_c_times_sqrt() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("π","√5", "Π[(π,1),(√5,1)]"),
      ("π","√π", "Π[(π,1),(√π,1)]"),
      // ("π","√π", "Π[(π,3/2)]"),
      ("e","√π","Π[(e^1),(√π^1)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  //
  // Sum
  //
  #[test]
  fn test_sum_times_sum() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("Σ(2,π)","Σ(2,π)", "Σ(4,Π(π,2))"),
      ("Σ[(1,ξ1),(1,√5)]","Σ(2,√5)","Σ[(2,√5),(20,ξ1)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  #[test]
  fn test_sum_times_prod() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("Σ(2,π)","Π(π,2)", "Σ(2,Π(π,3))"),
      ("Σ[(1,ξ1),(1,√5)]","Σ(2,√5)","Σ[(2,√5),(20,ξ1)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  #[test]
  fn test_sum_times_sqrt() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("Σ(2,π)","√5","Σ(2,Π[(π,1),(√5,1)]"),
      ("Σ(2,√5)","√5","Σ(10,ξ1)"),
      ("Σ(2,√5)","√5","ξ10"),
      ("Σ[(1,ξ1),(1,√5)]","√5","Σ[(1,√5),(5,ξ1)]")
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  //
  // Prod
  //
  #[test]
  fn test_prod_times_prod() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("Π(π,2)","Π(π,2)","Π(π,4)"),
      ("Π[(π,1),(√5,1)]","Π(π,2)","Π[(π,3),(√5,1)]"),
      ("Π(π,1/3)","Π(π,2/3)","Π(π,1)"),
      ("Π(π,1/3)","Π(π,2/3)","π"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  #[test]
  fn test_prod_times_sqrt() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("Π(π,2)","√π","Π[(π,2),(√π,1)]"),
      ("Π(π,1/4)","√π","Π(π,3/4)"),
      ("Π[(π,2),(√5,1)]","√5","Σ(5,Π(π,2))"),
      ("Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }
  // 
  // Sqrt
  //
  #[test]
  fn test_sqrt_times_sqrt() {
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("√5","√5","ξ5"),
      ("√10","√5","Σ(2,√5)"),
      ("√Σ[(1,ξ1),(1,√5)]","√Σ[(1,ξ1),(1,√5)]","Σ[(1,ξ1),(1,√5)]"),
      ("√Σ(2,π)","√π","Π[(π,1),(√2,1)]"),
      ("√Σ(6,π)","√2","Σ(2,√Σ(3,π))"),
      ("√Σ(2,π)","√2","Σ(2,√π)"),
      ("√Σ(6,π)","√2","Σ(2,√Σ(3,π))"),
      // ("√Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = Expr::parse_display(  a.to_string()).unwrap();
      let e_b   = Expr::parse_display(  b.to_string()).unwrap();
      let e_ans = Expr::parse_display(ans.to_string());
      assert_eq!(Ok(e_a.clone() * e_b.clone()) , e_ans);
      assert_eq!(Ok(e_b * e_a) , e_ans);
    }
  }

  // fn test_sum_times_sum() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }
  // #[test]
  // fn test_sum_times_prod() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }

  // #[test]
  // fn test_sum_times_sqrt() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }

  // #[test]
  // fn test_prod_times_prod() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }
  // #[test]
  // fn test_prod_times_sqrt() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }

  // #[test]
  // fn test_sqrt_times_sqrt() {
  //   assert_eq!(Expr:: * Expr:: , Expr::parse_display("".to_string()));
  // }
}
