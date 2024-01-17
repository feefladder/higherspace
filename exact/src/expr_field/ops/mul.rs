use std::{ops::Mul, fs};

use crate::expr_field::{Expr, FieldTrait, ExprTrait};

impl<'a, Field: FieldTrait<'a>> Mul for Expr<'a, Field> {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Expr::InDet(_),_) => self,
      (_,Expr::InDet(_)) => self.field().gulp(rhs),
      (Expr::Infty(fs, s1), Expr::Infty(_, s2)) => Expr::Infty(&fs, s1*s2),
      (Expr::Infty(_, _),_) => self,
      (_,Expr::Infty(_, _)) => self.field().gulp(rhs),
      (Expr::Zero(_),_) => self,
      (_,Expr::Zero(_)) => self.field().gulp(rhs),
      (Expr::One(fs),_) => fs.gulp(rhs),
      (_,Expr::One(_)) => self,
      (Expr::Val(rs),Expr::Val(rr)) => {
        rs.field.add_val(Field::get_val(rs) * Field::get_val(rr))
      },
      (Expr::Val(rs),Expr::Sum(rr)) => {
        rs.field.add_svec(Field::get_sum(rr) * Field::get_val(rs))
      }
      _ => todo!()
    }
  }
}