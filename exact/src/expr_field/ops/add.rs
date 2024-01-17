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
      (Expr::Val(rs), Expr::Val(rr)) => {
        self.field().add_val(Field::get_val(rs) + Field::get_val(rr))
      },
      (Expr::Val(rs),Expr::One(_)) => {
        self.field().add_val(Field::get_val(rs) + F::one())
      }
      (Expr::One(_),Expr::Val(rr)) => {
        rhs.field().add_val(Field::get_val(rr) + F::one())
      }
      (Expr::Val(rs),Expr::Sum(rr)) => {
        let mut sv = Field::get_sum(rr).terms.clone();
        sv.push((Field::get_val(rs),Expr::One(rs.field)));
        rs.field.add_svec(collect_like_terms(
          sv
        ))
      }
      (Expr::Sum(rs),Expr::Val(rr)) => {
        let mut sv = Field::get_sum(rs).terms.clone();
        sv.push((Field::get_val(rr),Expr::One(rs.field)));
        rs.field.add_svec(collect_like_terms(
          sv
        ))
      }
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

#[cfg(test)]
mod test_expr_field_add
{
  use super::*;
  use crate::expr_field::structs::type_field::TypeField;

  #[test]
  fn test_add_zero() {
    let f = TypeField::default();
    let tv: Vec<_> = vec![
      ("I", "O", "I"),
      ("I","-1", "O"),
      ("π", "O", "π"),
      ("π","Σ(-1,π)", "O"),
      ("Σ[(1,I),(1,π)]","ξ-1","π"),
      ("Σ[(1,I),(1,π)]","Σ(-1,π)","I"),
    ].iter().map(|(a,b,ans)|(f.parse(a),f.parse(b),f.parse(ans))).collect();
    // assert_eq!(Ok(Expr::from(Vec::<(F,Expr)>::new())),f.parse(""));
    for (a,b ,ans) in tv {
      println!("testing {}+{}={}",a,b,ans);
      assert_eq!(a + b , ans);
      println!("testing {}+{}={}",b,a,ans);
      assert_eq!(b + a , ans);
    }
  }

  #[test]
  fn test_val_plus_val() {
    let f = TypeField::default();
    let test_vec: Vec<(&str,&str,&str)> = vec![
      ("I", "I", "ξ2"),
      ("I", "ξ3", "ξ4"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = f.parse(  a);
      let e_b   = f.parse(  b);
      let e_ans = f.parse(ans);
      println!("{}+{}={}",a,b,ans);
      println!("{}+{}={}",e_a,e_b,e_ans);
      assert_eq!(e_a + e_b , e_ans);
      println!("{}+{}={}",b,a,ans);
      println!("{}+{}={}",e_b,e_a,e_ans);
      assert_eq!(e_b + e_a , e_ans);
    }
  }


  #[test]
  fn test_val_plus_sum() {
    let f = TypeField::default();
    let test_vec: Vec<(&str,&str,&str)> = vec![
      // ("ξ2", "Σ(3,π)", "Σ[(3,π),(2,I)]"),
      // ("ξ2", "Σ[(3,π),(2,I)]", "Σ[(3,π),(4,I)]"),
    ];
    for (a,b ,ans) in test_vec {
      let e_a   = f.parse(  a);
      let e_b   = f.parse(  b);
      let e_ans = f.parse(ans);
      assert_eq!(e_a + e_b , e_ans);
      assert_eq!(e_b + e_a , e_ans);
    }
  }
}