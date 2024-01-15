use std::ops::Add;

use crate::{F,One,expr_field::{Expr, FieldTrait, simplify::sum::collect_like_terms, ExprTrait}};

impl<
  'a,
  Field: FieldTrait<'a>,
> Add for Expr<'a, Field>
 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    let rhs_field = self.field().gulp(rhs);
    match (self, rhs_field) {
      // r.gulp: add expr to self field
      (Expr::Zero(_), _) => {rhs_field},
      // nothing to be done
      (_, Expr::Zero(_)) => {self},
      // (Expr::One(r), _) => {},
      // (Expr::InDet(_), _) => {self},
      // (_, Expr::InDet(_)) => {Expr::InDet(self.get_ref())}
      (Expr::Infty(_, s1), Expr::Infty(_, s2)) => {
        if s1 == s2 {self} else {Expr::InDet(self.field())}
      },
      (Expr::Infty(_, _), _) => {self},
      (_, Expr::Infty(_, _)) => {rhs_field},
      // (Expr::Val(r, vs), Expr::Val(_, vr)) => {Expr::Val(r, vs+vr)},
      // (Expr::Val(r, vs),Expr::Const(_, ))
      (Expr::Sum(r_s),Expr::Sum(r_r)) => {
        self.field().add_svec(
          collect_like_terms(
            [Field::get_sum(r_s).terms.clone(), Field::get_sum(r_r).terms.clone()].concat()
          )
        )
      }
      (Expr::Sum(r_s), _) => {
        // r_s.add_sum(s_s + r_s.gulp(rhs))
        // let ex_r = r_s.gulp(rhs);
        let mut s_s = Field::get_sum(r_s).terms.clone();
        s_s.push((F::one(),rhs_field));
        self.field().add_svec(collect_like_terms(s_s))
      },
      (_, Expr::Sum(r_r)) => {
        let mut sum_r = Field::get_sum(r_r).terms.clone();
        sum_r.push((F::one(), self));
        let res = collect_like_terms(sum_r);
        self.field().add_svec(res)
      }
      (_,_) => {
        let res = collect_like_terms(vec![(F::one(), self), (F::one(), rhs_field)]);
        self.field().add_svec(res)
      }
    }
  }
}