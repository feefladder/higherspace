use num_traits::One;
use appendlist::AppendList;
use crate::expr_field::{
  Expr, F,
  FieldRef,
  FieldTrait, ExprTrait, ExprFn, ExprFnTrait, SVec,
  structs::{
    Const,
    Sum,
    Prod,
  }, PVec
};

pub type TypeRef<'a> = FieldRef<'a, TypeField<'a>>;
pub type TypeExpr<'a> = Expr<'a, TypeField<'a>>;

#[derive(Debug, Default, PartialEq)]
pub struct TypeField<'a>{
  vals: AppendList<F>,
  consts: AppendList<Const>,
  sums: AppendList<Sum<'a, Self>>,
  prods: AppendList<Prod<'a, Self>>,
  fns: AppendList<ExprFn<'a, Self>>,
}

impl<'a> TypeField<'a> {
  #[inline]
  fn maybe_add<T: PartialEq>(v: &AppendList<T>, t: T) -> usize {
    match v.iter().position(|a_t| *a_t == t) {
      Some(i) => i,
      None => {
        v.push(t);
        v.len()-1
      }
    }
  }

}

impl<'a> FieldTrait<'a> for TypeField<'a> {
  // fn new() -> Self {
  //   TypeField::default()
  // }

  #[inline]
  fn get_val(r: FieldRef<'a, Self>) -> &'a F {
    &r.field.vals[r.index]
  }

  #[inline]
  fn get_const(r: FieldRef<'a, Self>) -> &'a Const {
    &r.field.consts[r.index]
  }

  #[inline]
  fn get_sum(r: FieldRef<'a, Self>) -> &'a Sum<'a, Self> {
    &r.field.sums[r.index]
  }

  #[inline]
  fn get_prod(r: FieldRef<'a, Self>) -> &'a Prod<'a,   Self> {
    &r.field.prods[r.index]
  }

  #[inline]
  fn get_fn(r: FieldRef<'a, Self>) -> &'a ExprFn<'a, Self> {
    &r.field.fns[r.index]
  }

  fn add_svec(&'a self, s: SVec<'a, Self>) -> Expr<'a, Self> {
    if s.len() == 0 {
      Expr::Zero(self)
    } else if s.len() == 1 && matches!(s[0].1,Expr::One(_))  {
      if s[0].0.is_one() {
        Expr::One(self)
      } else {
        Expr::Val(FieldRef { 
          field: self,
          index: TypeField::maybe_add(&self.vals, s[0].0)
        })
      }
    } else {
      Expr::Sum(FieldRef{
        field: self,
        index: TypeField::maybe_add(&self.sums, Sum { terms: s })
      })
    }
  }

  fn add_pvec(&'a self, p: PVec<'a, Self>) -> Expr<'a, Self> {
    if p.len() == 0 {
      Expr::One(self)
    } else {
      Expr::Prod(FieldRef { 
        field: self,
        index: TypeField::maybe_add(&self.prods, Prod { factors: p}) }
      )
    }
  }

  fn gulp(&'a self, expr: TypeExpr<'a>) -> TypeExpr<'a>{
    if &expr.field() == &self {
      //all done!
      expr
    } else {
      match expr {
        // recursive exprs: recursively add to this field
        Expr::Sum(r) => {
          let mut sum = TypeField::get_sum(r).clone();
          for (_, f) in sum.terms.iter_mut() {
            *f = self.gulp(*f);
          }
          Expr::Sum(TypeRef{
            field: self,
            index: TypeField::maybe_add(&self.sums, sum)
          })
        },
        Expr::Prod(r) => {
          let mut prod: Prod<'a, TypeField> = TypeField::get_prod(r).clone();
          for (b, _) in prod.factors.iter_mut() {
            *b = self.gulp(*b);
          }
          Expr::Prod(TypeRef{
            field: self,
            index: TypeField::maybe_add(&self.prods, prod)
          })
        },
        Expr::Fn(r) => {
          let mut f  = TypeField::get_fn(r).clone();
          f.set_inner(self.gulp(f.inner()));
          Expr::Fn(TypeRef{
            field: self,
            index: TypeField::maybe_add(&self.fns, f)
          })
        },
        // leaf exprs: directly add
        Expr::Val(r) => {
          Expr::Val(TypeRef{
            field: self,
            index: TypeField::maybe_add(
              &self.vals,
              r.field.vals[r.index]
            )
          })
        },
        Expr::Const(r) => {
          Expr::Const(TypeRef{
            field: self,
            index: TypeField::maybe_add(
              &self.consts,
              r.field.consts[r.index].clone()
            )
          })
        },
        // "fundamentals": just set ref
        Expr::Zero(_) => Expr::Zero(&self),
        Expr::One(_) => Expr::One(&self),
        Expr::InDet(_) => Expr::InDet(&self),
        Expr::Infty(_, s) => Expr::Infty(&self, s),
      }
    }
  }

  fn parse(input: &str) -> Expr<'a, Self> {
      todo!()
  }
}



// impl<'a> PartialEq<Vec<&str>> for TypeField<'a> {
//   fn eq(&self, other: &Vec<&str>) -> bool {
//     if self.exprs.borrow().len() != other.len() {
//       false
//     } else {
//       for i in 0..(other.len()-1) {
//         if format!("{}", self.exprs.borrow()[i]) != other[i] {
//           return false;
//         }
//       }
//       true
//     }
//   }
// }