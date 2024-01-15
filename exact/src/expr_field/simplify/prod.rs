use crate::{
  F,One,
  expr_rc::{Expr, PVec}
};

/// Prod+Sum: collect like terms
/// Π[(π,a),(π,b)] -> Π(π,a+b)
/// Π[(π,1),(e,2),(π,3)]
// Π[(2,1/2),(3,1/2)] -> Π(6,1/2)
#[inline]
pub fn collect_like_factors(v: PVec) -> PVec {
  let mut new_vec: PVec = PVec::new();
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

pub fn convert_to_sqrt(v: &mut PVec) {
  for (f, exp) in v.iter_mut() {
    if *exp == F::new(1u8,2u8) {
      *exp = F::one();
      *f = Expr::sqrt_expr(f.clone());
    }
  }
}