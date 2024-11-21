use num_traits::{One, Zero};
use core::cell::{RefCell, Ref};
// use appendlist::AppendList;
use crate::{
  F,
  expr_field::{
    Expr, FieldRef, FieldTrait, ExprTrait, ExprFn, ExprFnTrait, SVec,
    structs::{
      Const,
      Sum,
      Prod,
    }, PVec, io::parse_display::{Scanner, pd_expr}, simplify::{sum::{svecs_mul, svec_mul, svec_powi}, prod::distribute_prods}
  }
};

pub type TypeRef<'a> = FieldRef<'a, TypeField<'a>>;
pub type TypeExpr<'a> = Expr<'a, TypeField<'a>>;

#[derive(Debug, Default)]
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
    } else if v.is_nan() {
      TypeExpr::InDet(self)
    } else if v.is_infinite() {
      match v.sign() {
        Some(s) => {TypeExpr::Infty(self, s)}
        _ => unreachable!()
      }
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
    } else if s.len() == 1 && s[0].0.is_one() {
      self.gulp(s[0].1)
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
    } else if p.len() == 1 && p[0].1.is_one() {
      self.gulp(p[0].0)
    } else {
      // assert!(p.is_sorted());
      Expr::Prod(FieldRef { 
        field: self,
        index: TypeField::maybe_add(&self.prods, Prod { factors: p}) }
      )
    }
  }

  fn add_fn(&'a self, f: ExprFn<'a, Self>) -> Expr<'a, Self> {
    Expr::Fn(FieldRef { field: self, index: TypeField::maybe_add(&self.fns, f) })
  }

  fn try_distribute_prod(r: FieldRef<'a, Self>) -> Option<Expr<'a, Self>> {
    // let v: Vec<(Expr<'_, TypeField<'_>>, F)> = ;
    let (sums, rem) = distribute_prods(TypeField::get_prod(r).factors.clone());

    // take out vals: Π[(π,2),(5,1)] -> Σ(5,Π(π,2))
    // actually for non-integer powers, it'd be better to keep
    // numbers as a vector of prime factors
    // Π[(2,1/3),(3,1/2)] <-> Π[(4,1/6),(27,1/6)] <-> Π[(108,1/6)]
    //                        collect ---->
    //                        <--distribute
    // mul_pvec_v(take_out_vals(&mut rem),sums);
    let rem_expr = r.field.add_pvec(rem);
    Some(r.field.add_svec(svec_mul(sums, rem_expr)))
  }

  fn gulp(&'a self, expr: TypeExpr<'a>) -> TypeExpr<'a>{
    if std::ptr::eq(expr.field(), self) {
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

    use crate::expr_field::{structs::{Const, type_field::TypeField}, FieldTrait};

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

  #[test]
  fn test_typeexpr_partialeq() {
    let f = TypeField::default();
    let tv = vec![
      ("I","1"),
      ("O","0"),
      // I expected this to not work
      ("Σ(2,π)","Σ(2,π)"),
      ("Σ[(1,I),(1,π)]","Σ[(1,I),(1,π)]"),
      ("Σ[(1,π),(1,I)]","Σ[(1,π),(1,I)]"),
      ("Σ[(1,π),(1,I)]","Σ[(1,I),(1,π)]"),
      ("Π(π,2)","Π(π,2)"),
      ("Π[(π,1),(e,1)]","Π[(π,1),(e,1)]"),
      ("Π[(π,1),(e,1)]","Π[(e,1),(π,1)]"),
    ];
    for (t, res) in tv {
      println!("testing eq for {}=?{}",f.parse(t),f.parse(res));
      assert_eq!(format!("{}",f.parse(t)),format!("{}",f.parse(res)))
    }
    // assert!(false)
  }


  #[test]
  fn test_typeexpr_partialeq_2fields() {
    let f1 = TypeField::default();
    let f2 = TypeField::default();
    let tv = vec![
      ("I","I"),
      ("O","O"),
      ("O","0"),
      ("I","1"),
      ("2","2"),
      ("π","π"),
      ("Σ(2,π)","Σ(2,π)"),
      ("Σ[(1,π),(1,I)]","Σ[(1,π),(1,I)]"),
      ("Σ[(1,π),(1,I)]","Σ[(1,I),(1,π)]"),
      ("Π(π,2)","Π(π,2)"),
      ("Π[(π,1),(e,1)]","Π[(π,1),(e,1)]"),
      ("Π[(π,1),(e,1)]","Π[(e,1),(π,1)]"),
      ("Σ(2,Π[(π,1),(e,1)])","Σ(2,Π[(e,1),(π,1)])"),
      ("Σ(2,Π[(π,2),(e,1)])","Σ(2,Π[(e,1),(π,2)])"),
    ];
    for (t, res) in tv {
      println!("testing eq for {}=?{}",t,res);
      println!("testing eq for {}=?{}",f1.parse(t),f2.parse(res));
      assert_eq!(f1.parse(t),f2.parse(res))
    }
  }
}