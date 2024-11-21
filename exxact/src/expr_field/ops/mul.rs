use std::ops::Mul;

use crate::{
  F, One,
  expr_field::{
    Expr, FieldTrait, ExprTrait, 
    simplify::{
      prod::simplify_pvec,
      sum::{simplify_svec, val_to_coeff, svec_mul}
    }, ExprFn, ExprFnTrait
  }
};

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
        let v: Vec<_>;
        {
          v = Field::get_sum(rr).terms.iter()
          .map(|(c,f)| (c*Field::get_val(rs),*f))
          .collect()
        }
        rs.field.add_svec(v)
      },
      (Expr::Sum(rs),Expr::Val(rr)) => {
        let v: Vec<_>;
        {
          v = Field::get_sum(rs).terms.iter()
          .map(|(c,f)| (c*Field::get_val(rr),*f))
          .collect()
        }
        rs.field.add_svec(v)
      },
      (Expr::Val(r),_) => {
        r.field.add_svec(vec![(Field::get_val(r),rhs)])
      },
      (_,Expr::Val(r)) => {
        self.field().add_svec(vec![(Field::get_val(r),self)])
      }
      (Expr::Sum(rs),Expr::Sum(rr)) => {
        let mut v_new = Vec::new();
        for (f_s, e_s) in &Field::get_sum(rs).terms {
          for (f_r, e_r) in &Field::get_sum(rr).terms {
            // simplification and adding to field happens in
            // e_s * e_r
            v_new.push((f_s*f_r,(*e_s)*(*e_r)))
          }
        }
        val_to_coeff(&mut v_new);
        rs.field.add_svec(simplify_svec(v_new))
      },
      // This is a hard problem: Factorization is hard, while distribution is easy
      // (1+π)(1-π) = 1-π² -> (1-π²)/(1+π) = 1-π <= long-tail division
      //  1+π\1  -π²=1-π+0/(1+π)=1-π
      //    __1+π----/ |
      //       -π-π²---/
      //     __-π-π²
      //         O-- yay!
      //  1+π\1  +π²=1-π+2π²/(1+π)
      //    __1+π----/ | |
      //       -π+π²---/ |
      //     __-π-π²     |
      //         2π²-----/ sadd
      // Π(Σ[2,π],1/3)*Σ[2,π] = Π(Σ[2,π],4/3)
      // The fundamental equivalence that should be noted is:
      // Π(Σ[2+π],4/3) <=> Σ[Π(2,4/3),Π(π,4/3)]
      //           <--- factorize
      //           distribute -->
      // and note somewhere (in the Field) that these are equivalent
      // example: let 2φ := 1+√5
      // then          φ² = 1+φ
      // thus        1+φ² = 2+φ --> dis(φ²)+1 -> 1+φ+1 -> 2+φ
      // However, there may be multiple possible factorizations for a specific sum
      // (1-π²)(1+π) -> factorize(1-π²) -- let's say we don't know the factorization
      (Expr::Sum(rs),Expr::Prod(rr)) => {
        // if known factorization doesn't work
        // if factorize(Field::get_sum(rs)) == Field::get_sum(rs) {

        // }
        let mut v = Field::get_prod(rr).factors.clone();
        v.push((self,F::one()));
        rs.field.add_pvec(simplify_pvec(v))
      },
      (Expr::Prod(rs),Expr::Sum(_)) => {
        let mut v = Field::get_prod(rs).factors.clone();
        v.push((rhs,F::one()));
        rs.field.add_pvec(simplify_pvec(v))
      }
      (Expr::Sum(rs),_) => {
        let mut v: Vec<_>;
        {
          v = Field::get_sum(rs).terms.clone()
        }
        rs.field.add_svec(svec_mul(v, rhs))
      },
      (_,Expr::Sum(rr)) => {
        let v: Vec<_>;
        {
          v = Field::get_sum(rr).terms.clone();
        }
        self.field().add_svec(svec_mul( v, self))
      },
      (Expr::Prod(rs),Expr::Prod(rr)) => {
        let mut v = Field::get_prod(rs).factors.clone();
        v.extend_from_slice(&Field::get_prod(rr).factors[..]);
        rs.field.add_pvec(simplify_pvec(v))
      },
      (Expr::Prod(rs), Expr::Fn(rr)) if matches!(Field::get_fn(rr), ExprFn::Sqrt(_)) => {
        let mut pv = Field::get_prod(rs).factors.clone();
        let sqrt_inner = Field::get_fn(rr).inner();
        match pv.iter().position(|(b,e)| *b == sqrt_inner && !e.denom().unwrap().is_one()) {
          Some(pos) => {
            pv[pos].1 += F::new(1u8, 2u8);
            return rs.field.add_pvec(pv);
          }
          None => {}
        }
        match pv.iter().position(|(b,e)| *b == rhs && e.is_one()) {
          Some(pos) => {
            pv[pos].0 = sqrt_inner;
            return rs.field.add_pvec(pv);
          }
          None => {
            pv.push((rhs,F::one()));
            rs.field.add_pvec(simplify_pvec(pv))
          }
        }
      },
      (Expr::Fn(rs), Expr::Prod(rr)) if matches!(Field::get_fn(rs), ExprFn::Sqrt(_)) => {
        let mut pv = Field::get_prod(rr).factors.clone();
        let sqrt = Field::get_fn(rs).inner();
        match pv.iter().position(|(b,e)| *b == sqrt && !e.denom().unwrap().is_one()) {
          Some(pos) => {
            pv[pos].1 += F::new(1u8, 2u8);
            rs.field.add_pvec(pv)
          }
          None => {
            pv.push((self,F::one()));
            rs.field.add_pvec(simplify_pvec(pv))
          }
        }
      }
      (Expr::Prod(r),_) => {
        let mut v = Field::get_prod(r).factors.clone();
        v.push((rhs,F::one()));
        r.field.add_pvec(simplify_pvec(v))
      },
      (_,Expr::Prod(r)) => {
        let mut v = Field::get_prod(r).factors.clone();
        v.push((self,F::one()));
        self.field().add_pvec(simplify_pvec(v))
      },
      (Expr::Fn(rs),Expr::Fn(rr)) if
        matches!(Field::get_fn(rs),ExprFn::Sqrt(_)) &&
        matches!(Field::get_fn(rr),ExprFn::Sqrt(_)) => {
          let es = Field::get_fn(rs).inner();
          let er = Field::get_fn(rr).inner();
          if es == er {es} else {
            rs.field.add_fn(ExprFn::Sqrt(es*er))
          }
      },
      _ => {
        if self == rhs {
          self.field().add_pvec(vec![(self,F::from(2))])
        } else {
          let mut v = vec![
            (self,F::one()),
            (rhs,F::one()),
          ];
          v.sort_unstable();
          self.field().add_pvec(v)
        }
      }
    }
  }
}

#[cfg(test)]
mod test_expr_field_mul {
    use crate::expr_field::{
      FieldTrait,
      structs::type_field::TypeField, ExprTrait,
    };

    #[inline]
    fn test_ans_is_a_times_b(ans_str: &str, v: Vec<(&str, &str)>) {
      let f = TypeField::default();
      let ans = f.parse(ans_str);
      let tv: Vec<_> = v.iter().map(|(a,b)| (f.parse(a),f.parse(b))).collect();
      for (a,b) in tv {
        println!("{}*{}={}",a,b,ans);
        assert_eq!(a*b,ans);
        println!("{}*{}={}",b,a,ans);
        assert_eq!(b*a,ans);
      }
      let f1 = TypeField::default();
      let f2 = TypeField::default();
      let tv: Vec<_> = v.iter().map(|(a,b)| (f1.parse(a),f2.parse(b))).collect();
      for (a,b) in tv {
        println!("{}*{}->{}",a,b,a);
        assert!(std::ptr::eq((a*b).field(), a.field()));
        println!("{}*{}->{}",b,a,b);
        assert!(std::ptr::eq((b*a).field(), b.field()));
      }
    }

  #[test]
  fn test_mul_inv() {
    test_ans_is_a_times_b("I", vec![
      ("ξ1","ξ1"),
      ("ξ2","ξ1/2"),
      ("ξ-2","ξ-1/2"),
      ("π","Π(π,-1)"),
      ("Σ[(1,ξ1),(1,π)]","Π(Σ[(1,ξ1),(1,π)],-1)"),
      ("Π(π,2)","Π(π,-2)"),
      // super panic
      ("Π[(π,1),(e,1)]","Π[(π,-1),(e,-1)]"),
      ("√2","Σ(1/2,√2)"),
    ]);
  }

  #[test]
  fn test_mul_indet() {
    
  }
  #[test]
  fn test_mul_inf() {

  }

  #[test]
  fn test_mul_zero() {

  }
  #[test]
  fn test_mul_one() {

  }

  fn test_a_times_b(v: Vec<(&str, &str, &str)>) {
    let f = TypeField::default();
    let tv: Vec<_> = v.iter().map(|(a,b,c)| (f.parse(a),f.parse(b),f.parse(c))).collect();
    for (a,b,ans) in tv {
      println!("{}*{}={}",a,b,ans);
      assert_eq!(a*b,ans);
      println!("{}*{}={}",b,a,ans);
      assert_eq!(b*a,ans);
    }
    let f1 = TypeField::default();
    let f2 = TypeField::default();
    let tv: Vec<_> = v.iter().map(|(a,b,_)| (f1.parse(a),f2.parse(b))).collect();
    for (a,b) in tv {
      println!("{}*{}->{}",a,b,a);
      assert!(std::ptr::eq((a*b).field(), a.field()));
      println!("{}*{}->{}",b,a,b);
      assert!(std::ptr::eq((b*a).field(), b.field()));
    }
  }

  #[test]
  fn test_v_times_v() {
    test_a_times_b(vec![
      ("2","3","6"),
    ])
  }

  #[test]
  fn test_v_times_c() {
    test_a_times_b(vec![
      ("2","π","Σ(2,π)"),
    ])
  }

  #[test]
  fn test_c_times_c() {
    test_a_times_b(vec![
      ("π","π","Π(π,2)"),
      ("π","e","Π[(e,1),(π,1)]"),
    ])
  }

  #[test]
  fn test_v_times_sum() {
    test_a_times_b(vec![
      ("ξ2","Σ(2,π)","Σ(4,π)"),
      ("ξ2","Σ[(1,π),(1,e)]","Σ[(2,π),(2,e)]"),
    ])
  }
  #[test]
  fn test_sum_times_sum() {
    test_a_times_b(vec![
      ("Σ(2,π)","Σ(2,π)","Σ(4,Π(π,2))"),
      ("Σ[(1,I),(1,π)]","Σ[(1,I),(1,π)]","Σ[(1,I),(2,π),(1,Π(π,2))]"),
      ("Σ[(1,ξ1),(1,√5)]","Σ(2,√5)","Σ[(2,√5),(10,ξ1)]"),
    ])
  }
  #[test]
  fn test_sum_times_prod() {
    test_a_times_b(vec![
      ("Σ(2,π)","Π(π,2)", "Σ(2,Π(π,3))"),
      ("Σ[(1,ξ1),(1,√5)]","Σ(2,√5)","Σ[(2,√5),(10,ξ1)]"),
    ])
  }

  #[test]
  fn test_mul_rest() {
    test_a_times_b(vec![
      ("Σ(2,π)","√5","Σ(2,Π[(π,1),(√5,1)]"),
      ("Σ(2,√5)","√5","Σ(10,ξ1)"),
      ("Σ(2,√5)","√5","ξ10"),
      ("Σ[(1,ξ1),(1,√5)]","√5","Σ[(1,√5),(5,ξ1)]"),
      ("Π(π,2)","Π(π,2)","Π(π,4)"),
      ("Π[(π,1),(√5,1)]","Π(π,2)","Π[(π,3),(√5,1)]"),
      ("Π(π,1/3)","Π(π,2/3)","Π(π,1)"),
      ("Π(π,1/3)","Π(π,2/3)","π"),
      ("Π(π,2)","√π","Π[(π,2),(√π,1)]"),
      ("Π(π,1/4)","√π","Π(π,3/4)"),
      ("Π[(π,2),(√5,1)]","√5","Σ(5,Π(π,2))"),
      ("Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
      ("√5","√5","ξ5"),
      ("√10","√5","Σ(2,√5)"),
      ("√Σ[(1,ξ1),(1,√5)]","√Σ[(1,ξ1),(1,√5)]","Σ[(1,ξ1),(1,√5)]"),
      ("√Σ(2,π)","√π","Π[(π,1),(√2,1)]"),
      ("√Σ(6,π)","√2","Σ(2,√Σ(3,π))"),
      ("√Σ(2,π)","√2","Σ(2,√π)"),
      ("√Σ(6,π)","√2","Σ(2,√Σ(3,π))"),
      // ("√Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
    ])
  }
}