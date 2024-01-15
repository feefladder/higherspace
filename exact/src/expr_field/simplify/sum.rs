use num_traits::Zero;

use crate::expr_field::{SVec, ExprTrait, FieldTrait, Expr};

use super::collect_like;

/// Collect like terms in a sum. Also removes empty terms
/// ```raw
/// Σ[(1,Ι),(-1,Ι)] -> Σ(0,Ι) -> Σ()
/// ```
pub fn collect_like_terms<'a, Field: FieldTrait<'a>> (v: SVec<'a, Field>) -> SVec<'a, Field>{
  let mut res = collect_like(v,
    |(_,e1),(_,e2)| e1 == e2,
    |(f1,_),(f2,_)| *f1+=f2);
  res.retain(|(c,_)| !c.is_zero());
  res
}


