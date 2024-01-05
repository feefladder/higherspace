use std::{rc::Rc, ops::Deref, fmt::{Display, Debug}};

use num_traits::{sign, PrimInt};
use prime_factorization::{Factorization, UInt};
use fraction::{Sign, Ratio};
use num::{Integer, integer::Roots};

use crate::{
  F,One,Zero,
  expr_rc::{Expr,GetExpr},
  structs::Prod,
};

type Pvec = Vec<(Expr, F)>;
/// From a prod-type vector: Pvec
/// Also simplifies
/// Contained products should already be simplified
impl From<Pvec> for Expr {
  fn from(mut v: Pvec) -> Self {
    // Prod: sqrt to power and expand products
    sqrt_to_power_expand_prods(&mut v);
    // Prod: expand exponents for vals
    // Π(ξ6,5/2) -> 36, Π(ξ6,1/2)
    // Π(ξ2,-1/2) -> 1/2 Π(ξ2,1/2)
    let mut coeff = take_out_vals(&mut v);
    println!("Coeff: {}", coeff);
    if coeff.is_nan() {return Expr::nan();}
    if coeff.is_infinite() {
      return if coeff.is_sign_positive() {Expr::infinity()} else {Expr::neg_infinity()};
    }
    // Collect like fractional powers
    // Π[(2,1/2),(3,1/2)] -> Π(2*3,1/2)
    v = collect_like_num_powers(v);
    // Prod+Sum: collect like terms
    v = collect_like_terms(v);
    // Prod+Sum: strip zeroes
    // Π(π,0) -> Π() -> ξ1
    // Σ(0,π) -> Σ() -> ξ1
    v.retain(|(_,f)| f != &F::zero());
    // Sqrt magic
    // Π(12,1/2) -> 2 Π(3,1/2)
    // Π(24,1/3) -> 2 Π(3,1/3)
    coeff *= take_roots(&mut v);
    convert_to_sqrt(&mut v);
    // 1*Π(π,1) -> π
    if v.len() == 0 {return Expr::val_frac(coeff);}
    let prod = if v.len() == 1 && v[0].1 == F::one() {
      v[0].0.clone()
    } else {
      Expr::Prod(Rc::new(Prod { factors: v }))
    };
    if coeff.is_one() {
      prod
    } else {
      Expr::from(vec![(coeff, prod)])
    }
  }
}

#[inline]
fn sqrt_to_power(v: &mut Pvec) {
  for (maybe_sqrt, exp) in v.iter_mut() {
    match maybe_sqrt {
        Expr::Sqrt(_) => {
          *maybe_sqrt = maybe_sqrt.get_expr().unwrap();
          *exp *= F::new(1u8, 2u8);
        }
        _ => {}
    }
  }
}

/// Prod: expand prods (?would be recursive if unsafe?)
/// Π(Π[(π,1),(e,1)],1/3),(e,1)) -> Π[(π,1/3),(e,4/3)]
/// First append: Π(Π[(π,1),(e,1)],1/3),(e,1),(π,1/3),(e,1/3))
/// then filter: Π((e,1),(π,1/3),(e,1/3))    ^--------------^
#[inline]
fn sqrt_to_power_expand_prods(v: &mut Pvec) {
  sqrt_to_power(v);
  for (maybe_prod, exp) in v.clone().iter() {
    match maybe_prod {
      Expr::Prod(p) => {
        for (fact, exp) in p.as_ref().clone().factors {
          match fact {
            Expr::Prod(p) => {
              let v_f = &mut p.factors.clone();
              sqrt_to_power_expand_prods(v_f);
              v.append(v_f);
            },
            _ => {
              v.push((fact, exp));
            }
          }
        }
      }
      _ => {}
    }
  }
  v.retain(|(f,_)| !matches!(f, &Expr::Prod(_)));
}

/// Prod+Sum: collect like terms
/// Π[(π,a),(π,b)] -> Π(π,a+b)
/// Π[(π,1),(e,2),(π,3)]
// Π[(2,1/2),(3,1/2)] -> Π(6,1/2)
#[inline]
fn collect_like_terms(v: Pvec) -> Pvec {
  let mut new_vec: Pvec = Vec::new();
  for (fact, exp) in v {
    match new_vec.iter_mut().find(|(f,_)| f== &fact) {
      Some((_, exp_mut)) => {
        *exp_mut += exp;
      },
      None => { 
        new_vec.push((fact, exp));
      }
    }
  }
  new_vec
}

fn collect_like_num_powers(vec: Pvec) -> Pvec {
  collect_like(
    vec, 
    |(f1,e1),(f2,e2)| matches!((f1,f2),(Expr::Val(_),Expr::Val(_))) && *e1==e2 ,
    |(f1,_),(f2,_)| *f1 = f1.clone() * f2)
}

#[inline]
fn collect_like<T: Clone>(vec: Vec<T>, mut fn_match: impl FnMut(&&mut T, T) -> bool, fn_add: impl Fn(&mut T,T)) -> Vec<T> {
  let mut new_vec: Vec<T> = Vec::new();
  for item in vec {
    match new_vec.iter_mut().find(|a|fn_match(a,item.clone())) {
      Some(matc) => {
        fn_add(matc, item);
      },
      None => {
        new_vec.push(item);
      }
    }
  }
  new_vec
}

#[inline]
fn take_roots(v: &mut Pvec) -> F {
  let mut coeff = F::one();
  for (f_expr, exp) in v.iter_mut() {
    match f_expr {
      Expr::Val(f_rc) => {
        match (**f_rc, &exp) {
          (F::Rational(f_sign, mut f_r), F::Rational(e_sgn, mut e_r )) => {
            assert_eq!(*e_sgn, Sign::Plus);
            assert!(e_r.fract() == e_r);
            coeff *= factor_root(&mut f_r, &mut e_r);
            *f_expr = Expr::val_frac(F::Rational(f_sign, f_r));
            *exp = F::Rational(Sign::Plus, e_r);
          },
          _ => {unreachable!()}
        }
      },
      _=> {}
    }
  }
  coeff
}

#[inline]
fn factor_root(base: &mut Ratio<u32>, exp: &mut Ratio<u32>) -> F {
  let mut coeff = F::one();
  let mut f_num = Factorization::run(*base.numer()).prime_factor_repr();
  let mut f_den = Factorization::run(*base.denom()).prime_factor_repr();
  // Π(54,1/2) -> 3 Π(6,1/2)
  // f_num: [(2,1),(3,3)]
  for (prime, p_exponent) in f_num.iter_mut() {
    let rem = (*p_exponent * exp.numer()) % exp.denom();
    let div = (*p_exponent * exp.numer()) / exp.denom();
    if div != 0  {
      coeff *= F::from(prime.pow(div));
      *p_exponent = rem;
    }
  }

  // Π(1/54,1/2) -> 1/3 Π(6,1/2)
  // f_den: [(2,1),(3,3)]
  for (prime, p_exponent) in f_den.iter_mut() {
    let rem = (*p_exponent * exp.numer()) % exp.denom();
    let div = (*p_exponent * exp.numer()) / exp.denom();
    if div != 0  {
      coeff /= F::from(prime.pow(div));
      *p_exponent = rem;
    }
  }

  coeff
}

#[inline]
fn neg_pow_sign(sign: Sign, pow: i32) -> Sign {
  if pow.is_odd() {sign} else {Sign::Plus}
}

/// Take out values from the equation
/// Π(ξ6,5/2) -> 36, Π(ξ6,1/2)
/// also deals with infinity and zero:
/// Π(ξ0,a)  ->  0(a!=∞) Nan(a==∞)
/// Π(ξα,0)  ->  nothing special
/// Π(ξa,∞)  ->  ∞(a>1) 0(a<1) nan(a=1)
/// Π(ξa,-∞) ->  ∞(a<1) 0(a>1) nan(a=1)
/// Π(ξ∞,a)  ->  ∞(a!=0) Nan(a==0)
/// Π(ξ-∞,a) -> -∞(a!=0) Nan(a==0)
/// also: ∞*0 and ∞*0 checks
/// Π(1,∞) -> Nan  Π(∞,0) -> Nan
/// Π()
fn take_out_vals(vect: &mut Pvec) -> F {
  println!("Taking vals: {:?}", vect);
  let mut coeff: F = F::one();
  for (fact_expr, exp_frac ) in vect.iter_mut() {
    match (&fact_expr, &exp_frac) {
      (_,_) if exp_frac.is_nan() => {
        return F::nan();
      }
      (Expr::Val(f),_) if f.is_nan() => {
        return F::nan();
      }
      // Π(ξ0,a)  ->  0(a!=∞) Nan(a==∞)
      (_,_) if fact_expr.is_zero() => {
        if coeff.is_infinite() {
          eprintln!("Encountered 0*∞!: {:?}", vect);
          return F::nan();
        }
        coeff *= F::zero();
      },
      // Π(ξf,∞)  ->  ∞(f>1) 0(f<1) nan(f=1)
      // Π(ξf,-∞) ->  ∞(f<1) 0(f>1) nan(f=1)
      (Expr::Val(f),_) if exp_frac.is_infinite() => {
        if f.is_one() {
          eprintln!("Encountered 1^∞!: {:?}", vect);
          return F::nan();
        }
        coeff *= if exp_frac.is_sign_positive() {
          if **f>F::one() {F::infinity()} else {F::zero()}
        } else {
          if **f>F::one() {F::zero()} else {F::infinity()}
        };
      },
      // Π(ξ∞,e)  ->  ∞(e!=0) Nan(e==0)
      // Π(ξ-∞,e) -> -∞(e!=0) Nan(e==0)
      (Expr::Val(f),_) if f.is_infinite() => {
        if exp_frac.is_zero() {
          eprintln!("Encountered ∞^0!: {:?}", vect);
          return F::nan();
        }
        coeff *= if f.is_sign_positive() {F::infinity()} else {F::neg_infinity()};
      }
      (
        Expr::Val(fact_f ),
        F::Rational(exp_sign, exp_r )
      ) => {
        match **fact_f {
          F::Rational(fact_sign, fact_r) => {
            println!("Doing r^r: {:?}^{:?}", fact_f, exp_frac);
            // rational ^ rational
            let e_sign_i = if matches!(exp_sign, Sign::Plus) {1} else {-1};
            let exp_int: i32 = exp_r
              .to_integer()
              .try_into().unwrap();
            println!("Doing r^r: {:?}^{:?}", e_sign_i, exp_int);
            let new_exp_frac:F = F::Rational(*exp_sign, exp_r.fract());
            let new_f_sign = neg_pow_sign(fact_sign, exp_int);
            // Π(ξ2,-1/2) -> 1/2, Π(ξ2,1/2)
            if *exp_sign == Sign::Minus {
              let exp_denom: i32 = (*
                new_exp_frac
                .denom()
                .unwrap())
                .try_into()
                .unwrap();
              coeff *= F::Rational(neg_pow_sign(fact_sign, exp_denom), fact_r.pow( exp_denom));
            }
            // 
            coeff *= F::Rational(neg_pow_sign(fact_sign, exp_int),fact_r.pow(e_sign_i*exp_int));
            *exp_frac = new_exp_frac;
          },
          _ => unreachable!()
        }
      }
      _ => {}
    } //
  } // for loop
  coeff
}

fn convert_to_sqrt(v: &mut Pvec) {
  for (f, exp) in v.iter_mut() {
    if *exp == F::new(1u8,2u8) {
      *exp = F::one();
      *f = Expr::sqrt_expr(f.clone());
    }
  }
}

#[cfg(test)]
mod test_from_prod_simplify{
  use super::*;
  const i: u32 = 5;

  #[test]
  fn test_filter_0() {
    let test_vec = vec![
      // Filter out 0s, if empty after, return one
      (vec![(Expr::c_pi(), F::zero())],Expr::one()),
      // SO
      (vec![
        (Expr::c_pi(), F::zero()),
        (Expr::c_e(), F::zero()),
      ],Expr::one()),
      // SO
      (vec![
        (Expr::c_pi(), F::from(2)),
        (Expr::c_e(), F::zero()),
      ],Expr::prod_pi_i(2)),
      // // if single value ^1 after, return value
      (vec![
        (Expr::c_pi(), F::one()),
        (Expr::c_e(), F::zero()),
      ],Expr::c_pi()),
    ];
    for (from, ans) in test_vec {
      assert_eq!(Expr::from(from), ans);
    }
  }

  #[test]
  fn test_strip_vals_to_val() {
    let test_vec = vec![
      // Π(ξ5,1) -> ξ5
      (vec![
        (Expr::val_i(i),F::one()),
      ],Expr::val_i(i)),
      // Π(ξ5,2) -> ξ25
      (vec![
        (Expr::val_i(i),F::from(2)),
      ],Expr::val_i(i*i)),
      // Π[(ξ5,1),(ξ2,1)] -> ξ10
      (vec![
        (Expr::val_i(i),F::one()),
        (Expr::val_i(2),F::one()),
      ],Expr::val_i(i*2)),
      // Π[(ξ5,1),(ξ2,2)] -> ξ20
      (vec![
        (Expr::val_i(i),F::one()),
        (Expr::val_i(2),F::from(2)),
      ],Expr::val_i(i*2*2)),
    ];
    for (from, ans) in test_vec {
      println!("Trying {:?} ?= {:?}", from, ans);
      assert_eq!(Expr::from(from), ans);
    }
  }
  #[test]
  fn test_strip_vals_to_expr() {
    let test_vec = vec![
    // Π[(ξ1,1),(π,1)] -> π
    (vec![
      (Expr::val_i(1), F::one()),
      (Expr::c_pi(), F::one()),
      ],Expr::c_pi()),
    // Π[(ξ2,1),(ξ1/2,1)(π,1)] -> π
    (vec![
      (Expr::val_i(2), F::one()),
      (Expr::val_frac(F::new(1u8,2u8)),F::one()),
      (Expr::c_pi(), F::one()),
    ],Expr::c_pi()),
    ];
    for (from, ans) in test_vec {
      assert_eq!(Expr::from(from), ans);
    }
  }
  #[test]
  fn test_strip_vals_to_sum() {
    let test_vec = vec![
      // Π[(ξ5,1),(π,2)] -> Σ(5,Π(π,2))
      (vec![
        (Expr::val_i(i), F::one()),
        (Expr::c_pi(), F::from(2)),
      ],Expr::from(vec![(F::from(i), Expr::prod_pi_i(2))])),
      // Π[(ξ5,1),(π,1)] -> Σ(5,π)
      (vec![
        (Expr::val_i(i), F::one()),
        (Expr::c_pi(), F::one()),
      ],Expr::from(vec![(F::from(i), Expr::c_pi())])),
      // Π[(ξ5,1),(π,2)] -> Σ(5,Π(π,2))
      (vec![
        (Expr::val_i(i), F::one()),
        (Expr::c_pi(), F::from(2)),
      ],Expr::from(vec![(F::from(i), Expr::prod_pi_i(2))])),
      // Π[(ξ5,2),(π,1)] -> Σ(25,π)
      (vec![
        (Expr::val_i(i), F::from(2)),
        (Expr::c_pi(), F::one())
        ],Expr::sum_i_pi(25)),
      // Π[(ξ5,1),(ξ2,1),(π,2)] -> Σ(10,Π(π,2))
      (vec![
        (Expr::val_i(i),F::one()),
        (Expr::val_i(2),F::one()),
        (Expr::c_pi(), F::from(2)),
      ],Expr::from(vec![(F::from(10),Expr::prod_pi_i(2))])),
      // Π[(ξ5,1),(ξ2,2),(π,1)] -> Σ(20,π)
      (vec![
        (Expr::val_i(i),F::one()),
        (Expr::val_i(2),F::from(2)),
        (Expr::c_pi(), F::one()),
      ],Expr::from(vec![(F::from(i*2*2),Expr::c_pi())])),
      // Π[(ξ5,1),(e,1),(ξ2,2),(π,1)] -> Σ(20,Π[(e,1),(π,1)])
    ];
    for (from, ans) in test_vec {
      assert_eq!(Expr::from(from), ans);
    }
  }
}