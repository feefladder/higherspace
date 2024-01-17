use std::rc::Rc;

use prime_factorization::Factorization;
use fraction::{Sign, Ratio};
use num::Integer;

use crate::{
  F,One,Zero,
  expr_rc::{
    Expr,
    GetExpr,
    simplify::{
      collect_like,
      prod::{
        collect_like_factors,
        convert_to_sqrt,
      }
    }, PVec
  },
  expr_rc::structs::Prod, FType,
};

use super::FromRaw;

impl FromRaw<PVec> for Expr {
  fn from_raw(v: PVec) -> Self {
    if v.len() == 0 {return Expr::one();}
    if v.len() == 1 && v[0].1 == F::one() {
      v[0].0.clone()
    } else {
      Expr::Prod(Rc::new(Prod { factors: v }))
    }
  }
}

/// From a prod-type vector: PVec
/// Also simplifies
/// Contained products should already be simplified
impl From<PVec> for Expr {
  fn from(mut v: PVec) -> Self {
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
    v = collect_like_factors(v);
    // Prod+Sum: strip zeroes
    // Π(π,0) -> Π() -> ξ1
    // Σ(0,π) -> Σ() -> ξ1
    v.retain(|(_,f)| !f.is_zero());
    if v.len() == 0 {
      return Expr::val_i(coeff);
    }
    // Sqrt magic
    // Π(12,1/2) -> 2 Π(3,1/2)
    // Π(24,1/3) -> 2 Π(3,1/3)
    coeff *= take_roots(&mut v);
    convert_to_sqrt(&mut v);
    // 1*Π(π,1) -> π
    if coeff == F::one() {
      Expr::from_raw(v)
    } else {
      Expr::from_raw(vec![(F::from(coeff),Expr::from_raw(v))])
    }
  }
}

#[inline]
fn sqrt_to_power(v: &mut PVec) {
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

/// Prod: expand prods (recursive)
/// Π(Π[(π,1),(e,1)],1/3),(e,1)) -> Π[(π,1/3),(e,4/3)]
/// First append: Π(Π[(π,1),(e,1)],1/3),(e,1),(π,1/3),(e,1/3))
/// then retain: Π((e,1),(π,1/3),(e,1/3))    ^--------------^
#[inline]
fn sqrt_to_power_expand_prods(v: &mut PVec) {
  sqrt_to_power(v);
  for (maybe_prod, _) in v.clone().iter() {
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


pub fn collect_like_num_powers(vec: PVec) -> PVec {
  collect_like(
    vec, 
    |(f1,e1),(f2,e2)| matches!((f1,f2),(Expr::Val(_),Expr::Val(_))) && *e1==e2 ,
    |(f1,_),(f2,_)| *f1 = f1.clone() * f2)
}


#[inline]
fn take_roots(v: &mut PVec) -> F {
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
  // Π(54,1/2) -> 3 Π(6,2/3)
  // f_num: [(2,1),(3,3)] <- 54
  // [(2,2),(3,6)]
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
fn neg_pow_sign<T: Integer>(sign: Sign, pow: T) -> Sign {
  if pow.is_odd() {sign} else {Sign::Plus}
}

/// Take out values from the equation
/// ```raw
/// Π(ξ6,5/2) -> 36, Π(ξ6,1/2)
/// Π(ξ8,1) -> 8, Π()
/// also deals with infinity and zero:
/// Π(ξ0,a)  ->  0(a!=∞) Nan(a==∞)
/// Π(ξα,0)  ->  1
/// Π(ξa,∞)  ->  ∞(a>1) 0(a<1) nan(a=1)
/// Π(ξa,-∞) ->  ∞(a<1) 0(a>1) nan(a=1)
/// Π(ξ∞,a)  ->  ∞(a!=0) Nan(a==0)
/// Π(ξ-∞,a) -> -∞(a!=0) Nan(a==0)
/// also: ∞*0 and ∞*0 checks:
/// Π(1,∞) -> Nan  Π(∞,0) -> Nan
/// Π()
/// ```
fn take_out_vals(vect: &mut PVec) -> F {
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
            println!("Doing r^r: {:?}^{:?}", fact_r, exp_int);
            let new_exp_frac:F = F::Rational(*exp_sign, exp_r.fract());
            let new_f_sign = neg_pow_sign(fact_sign, exp_int);
            coeff *= F::Rational(new_f_sign, fact_r.pow(e_sign_i*exp_int));
            // Π(ξ2,-1/2) -> 1/2, Π(ξ2,1/2)
            // Π(ξ2,-1/2) -> ξ2.pow(2/1), Π(ξ2,1/2)
            // Π(ξ-2,-2/3) -/-> 2.pow(3/2) Π(ξ2,2/3)
            // 
            coeff *= div_by_root_to_f_times_root_raw(fact_sign, fact_r, *new_exp_frac.denom().unwrap());
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

/// ```raw
/// Π(ξ2,-1/2) -> 1/2, Π(ξ2,1/2)
/// Π(ξ2,-1/2) -> ξ2.pow(2/1)* Π(ξ2,1/2)
/// Π(ξ-2/5,-1/3) -> (-2/5).pow(3)* Π(ξ-2/5,1/3)
/// ```
pub fn div_by_root_to_f_times_root(base: F, exp:&mut F) -> F {
  match (base, exp) {
    (F::Rational(b_sgn, b_r),F::Rational(_, e_r)) => {
      if *e_r.numer() != 1 || b_sgn == Sign::Plus {
        F::one()
      } else {
        div_by_root_to_f_times_root_raw(b_sgn, b_r, *e_r.denom())
      }
    },
    _ => todo!()
  }
}

/// ```raw
/// Π(ξ2,-1/2) -> 1/2, Π(ξ2,1/2)
/// Π(ξ2,-1/2) -> ξ2.pow(2/1)* Π(ξ2,1/2)
/// Π(ξ-2/5,-1/3) -> (-2/5).pow(3)* Π(ξ-2/5,1/3)
///    ||      \-exp_den
///    |\-b_r
///    \-b_sgn
/// ```
fn div_by_root_to_f_times_root_raw(b_sgn:Sign, b_r: Ratio<FType>, exp_den: FType) -> F {
  if exp_den == 1 {
    F::one()
  } else {
    F::Rational(neg_pow_sign(b_sgn, exp_den), b_r.pow(exp_den.try_into().unwrap()))
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
      println!("trying {:?} ?= {}", from, ans);
      assert_eq!(Expr::from(from), ans);
    }
  }
}