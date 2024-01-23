use num::Integer;
use num_traits::{Zero, PrimInt};

use crate::{
  F,
  expr_field::{
  Expr,
  SVec,
  simplify::collect_like, structs::sum::sort_svec, FieldTrait,
}, FType};

/// Changes values in a sum vector to coefficients
/// Does not deal specially with Nan and infinity
/// ```raw
/// Σ(5,2)->Σ(10,I)
/// ```
pub fn val_to_coeff<'a, Field: FieldTrait<'a>>(v: &mut SVec<'a, Field>) {
  for (c, f) in v.iter_mut() {
    match f {
      Expr::Val(r) => {
        *c *= Field::get_val(*r);
        *f = Expr::One(r.field);
      }
      _ => {}
    }
  }
}

/// Collect like terms in a sum. Also removes empty terms
/// ```raw
/// Σ[(1,Ι),(-1,Ι)] -> Σ(0,Ι) -> Σ()
/// ```
pub fn simplify_svec<'a, Field> (v: SVec<'a, Field>) -> SVec<'a, Field>
where Expr<'a, Field>: Ord
{
  let mut res = collect_like(v,
    |(_,e1),(_,e2)| e1 == e2,
    |(f1,_),(f2,_)| *f1+=f2);
  res.retain(|(c,_)| !c.is_zero());
  sort_svec(&mut res);
  res
}


/// Multiply a sum with an expr
/// ```raw
/// Σ[1,π]*π -> Σ[π,π²]
/// Σ[1,√5]*√5 -> Σ[5,√5]
/// ```
/// You *could* use this to multiply with a value, inf or indet, but pls don't
pub fn svec_mul<'a, Field: FieldTrait<'a>>(mut svec: SVec<'a, Field>, expr: Expr<'a, Field>) -> SVec<'a, Field> {
  // let mut res = svec.clone();
  for (_, f) in svec.iter_mut() {
    *f = *f*expr;
  }
  val_to_coeff(&mut svec);
  simplify_svec(svec)
}

/// Multiply an svec with a value
/// ```raw
/// Σ[1,π]*2 -> Σ[2,(2,π)]
/// ```
// pub fn svec_mul_val<'a, Field, T>(mut svec: SVec<'a, Field>, val: T)
// where F: From<T>
// {
//   for (c,_) in svec.iter_mut() {
//     *c *= val;
//   }
// }

/// Multiply two sums:
/// ```raw
/// Σ[1,π]*Σ[1,-π]->Σ[1,-π²]
/// Σ[1,√5]*Σ[1,√5]->Σ[6,2√5]
/// ```
pub fn svecs_mul<'a, Field: FieldTrait<'a>> (a: &SVec<'a, Field>, b: &SVec<'a, Field>) -> SVec<'a, Field> {
 let mut res = Vec::new();
 for (ca, fa) in a {
  for (cb, fb) in b {
    res.push((ca*cb, (*fa)*(*fb)));
  }
 }
 val_to_coeff(&mut res);
 simplify_svec(res)
}

/// Raise a sum to an integer power
/// ```raw
/// Σ[1,√5]²->Σ[6,2√5]
/// ```
pub fn svec_powi<'a, Field: FieldTrait<'a>> (base: &SVec<'a, Field>, exp: &FType) -> SVec<'a, Field>{
  let mut res = base.clone();
  for _ in 1..*exp {
    res = svecs_mul(&res, &base)
  }
  res
}
