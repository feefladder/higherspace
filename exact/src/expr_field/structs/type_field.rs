use num_traits::{One, Zero};
use core::cell::{RefCell, Ref};
// use appendlist::AppendList;
use crate::expr_field::{
  Expr, F,
  FieldRef,
  FieldTrait, ExprTrait, ExprFn, ExprFnTrait, SVec,
  structs::{
    Const,
    Sum,
    Prod,
  }, PVec, io::parse_display::{Scanner, pd_expr}
};

pub type TypeRef<'a> = FieldRef<'a, TypeField<'a>>;
pub type TypeExpr<'a> = Expr<'a, TypeField<'a>>;

#[derive(Debug, Default, PartialEq)]
pub struct TypeField<'a>{
  vals: RefCell<Vec<F>>,
  consts: RefCell<Vec<Const>>,
  sums: RefCell<Vec<Sum<'a, Self>>>,
  prods: RefCell<Vec<Prod<'a, Self>>>,
  fns: RefCell<Vec<ExprFn<'a, Self>>>,
}

impl<'a> TypeField<'a> {
  #[inline]
  pub fn maybe_add<T: PartialEq>(v: &RefCell<Vec<T>>, t: T) -> usize {
    let index: Option<usize> ;
    {
      index = v.borrow().iter().position(|a_t| *a_t == t)
    }
    match index {
      Some(i) => i,
      None => {
        v.borrow_mut().push(t);
        v.borrow().len()-1
      }
    }
  }
}

impl<'a> FieldTrait<'a> for TypeField<'a> {
  // fn new() -> Self {
  //   TypeField::default()
  // }

  #[inline]
  fn get_val(r: FieldRef<'a, Self>) -> F {
    r.field.vals.borrow()[r.index]
  }

  #[inline]
  fn get_const(r: FieldRef<'a, Self>) -> Const {
    r.field.consts.borrow()[r.index]
  }

  #[inline]
  fn get_sum(r: FieldRef<'a, Self>) -> Ref<'a, Sum<'a, Self>> {
    Ref::map(r.field.sums.borrow(), |v| &v[r.index])
  }

  #[inline]
  fn get_prod(r: FieldRef<'a, Self>) -> Ref<'a, Prod<'a,   Self>> {
    Ref::map(r.field.prods.borrow(), |v| &v[r.index])
  }

  #[inline]
  fn get_fn(r: FieldRef<'a, Self>) -> ExprFn<'a, Self> {
    r.field.fns.borrow()[r.index].clone()
  }

  fn add_val(&'a self, v: F) -> Expr<'a, Self> {
    if v.is_one() {
      TypeExpr::One(self)
    } else if v.is_zero() {
      TypeExpr::Zero(self)
    } else {
      TypeExpr::Val(FieldRef{field: self, index: TypeField::maybe_add(&self.vals, v)})
    }
  }

  fn add_const(&'a self, c: Const) -> Expr<'a, Self> {
    TypeExpr::Const(FieldRef{field: self, index: TypeField::maybe_add(&self.consts, c)})
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

  fn add_fn(&'a self, f: ExprFn<'a, Self>) -> Expr<'a, Self> {
    Expr::Fn(FieldRef { field: self, index: TypeField::maybe_add(&self.fns, f) })
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
              TypeField::get_val(r)
            )
          })
        },
        Expr::Const(r) => {
          Expr::Const(TypeRef{
            field: self,
            index: TypeField::maybe_add(
              &self.consts,
              TypeField::get_const(r)
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

  fn parse(&'a self, input: &str) -> Expr<'a, Self> {
    let mut sc = Scanner::new(input, self);
    pd_expr(&mut sc)
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

#[cfg(test)]
mod test_typefield
{
    use ordered_float::NotNan;

    use crate::expr_field::structs::Const;

    use super::TypeField;

  #[test]
  fn test_typefield_maybe_add() {
    let f = TypeField::default();
    assert_eq!(TypeField::maybe_add(
      &f.consts,
      Const{ch: 'π', ascii: "pi", f64: NotNan::new(core::f64::consts::PI).unwrap()}
    ), 0);
    assert_eq!(TypeField::maybe_add(
      &f.consts,
      Const{ch: 'π', ascii: "pi", f64: NotNan::new(core::f64::consts::PI).unwrap()}
    ), 0);
    assert_eq!(TypeField::maybe_add(
      &f.consts,
      Const{ch: 'e', ascii: "pi", f64: NotNan::new(core::f64::consts::E).unwrap()}
    ), 1);
  }
}