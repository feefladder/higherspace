use crate::{
  F,One, Zero,
  expr_field::{Expr, ExprTrait, PVec, SVec, FieldTrait, ExprFn,
    simplify::sum::{
      svec_powi,
      svecs_mul,
    }
  }
};

use super::collect_like;

pub fn simplify_pvec<'a, Field>(v: PVec<'a, Field>) -> PVec<'a, Field>
where Expr<'a, Field>: Ord
{
  let mut res = collect_like_factors(v);
  res.retain(|(_,e)| *e!=F::zero());
  res.sort_unstable();
  res
}

/// Prod+Sum: collect like terms
/// Π[(π,a),(π,b)] -> Π(π,a+b)
/// Π[(π,1),(e,2),(π,3)]
// Π[(2,1/2),(3,1/2)] -> Π(6,1/2)
#[inline]
pub fn collect_like_factors<'a, Field: 'a>(v: PVec<'a, Field>) -> PVec<'a, Field>
where Expr<'a,Field>: PartialEq
{
  collect_like(v,
    |(b1,_),(b2,_)| b1 == b2 ,
    |(_,e1),(_,e2)| *e1 += e2
  )
}

#[inline]
pub fn convert_to_sqrt<'a, Field: FieldTrait<'a>>(v: &mut PVec<'a, Field>) {
  for (base, exp) in v.iter_mut() {
    if *exp == F::new(1u8,2u8) {
      *exp = F::one();
      *base = base.field().add_fn(ExprFn::Sqrt(*base));
    }
  }
}


/// Distribute products to a sum
/// ```raw
/// Π[Σ[1,π],Σ[1,-π]] -> Σ[1,-π²]
/// ```
/// currently non-recursive. E.g.
/// ```raw
/// Π[Σ[1,√Σ[1,√5],Σ[1,-√Σ[1,√5]]] -> Σ[1,-Σ[1,√5]] -/-> -√5
/// (1+√(1+√5))*(1-√(1+√5)) = 1-(1+√5) = -√5
/// ```
/// that's not recursion, that's taking out products. Let me try again...
/// ```raw
/// Π[Σ[1,Π[Σ[1,π],Σ[1,-π]]],Σ[1,-Π[Σ[1,π],Σ[1,-π]]]] -> 
/// Σ[1,-Π[(Σ[1,π],2),(Σ[1,-π],2)]] ->
/// Σ[1,-1,2π,π²] -> Σ[2π,π²]
/// ```
/// maybe DFS is a better approach?
/// also, these factorizations for which recursion is needed are ugly and not really the point of factorization?
pub fn distribute_prods<'a, Field: FieldTrait<'a>>(mut v: PVec<'a, Field>) -> (SVec<'a, Field>, PVec<'a, Field>) {
  // check if there are sums in here with integer powers
  let (sums,not_sums): (Vec<(Expr<'_, Field>,F)>,Vec<_>) = v
    .iter()
    .partition(
      |(b,e)|
      matches!(b, Expr::Sum(_)) && e.denom().unwrap().is_one()
    );

  if sums.len() == 0 {return (Vec::new(),v);}

  // let mut res: SVec<'a, Field> = vec![(F::one(),Expr::One(r.field))];
  let (Expr::Sum(r), e) = sums[0] else {panic!("not a sum!")};
  // let s = ;
  let mut res = svec_powi(&Field::get_sum(r).terms, e.numer().unwrap());
  for (s,e) in &sums[1..] {
    let Expr::Sum(r) = s else {panic!("not a sum!")};
    res = svecs_mul(&res, &svec_powi(&Field::get_sum(*r).terms, e.numer().unwrap()))
  }

  
  (res,not_sums)
}