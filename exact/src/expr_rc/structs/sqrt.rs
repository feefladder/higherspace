use std::rc::Rc;
use std::ops::{Mul, Add};

use crate::enums::{F, Expr, VOrRef};

#[derive(Debug, PartialEq, Clone)]
pub struct Sqrt(VOrRef);


impl Mul for Sqrt {
  type Output = Rc<Expr>;
  fn mul(self, rhs: Self) -> Self::Output {
      match (self, rhs) {
        (s, r) if s == r => {
          match s {
            Sqrt(VOrRef::Val(v)) => {
              Rc::new(Expr::Val(v))
            },
            Sqrt(VOrRef::Ref(r)) => {
              // Not sure how this would work if there are multiple references to r
              r
            }
          }
        },
        (Sqrt(VOrRef::Val(v_s)), Sqrt(VOrRef::Val(v_r))) => {
          Rc::new(Expr::Sqrt(Sqrt(VOrRef::Val(v_s*v_r))))
        },
        (Sqrt::Val(v_s), Sqrt::Ref(r_r)) => {
          // 
          Rc::new(Expr::Sqrt(Sqrt::Ref(r_r*Expr::Val(v_s))))
        },
        (Sqrt::Ref(r_s), Sqrt::Val(v_r)) => {
          Rc::new(Expr::Sqrt(Sqrt{v:Sqrt::Ref(r_s*Expr::Val(v_r))}))
        },
        (Sqrt::Ref(r_s), Sqrt::Ref(r_r)) => {
          todo!("ref sqrt calc");
          // Expr::Val(F::from(5))
        }
      }
  }
}

#[test]
fn sqrt_mul() {
  let s = Sqrt{v:Sqrt::Val(F::from(5))};
  assert_eq!(&*(s.clone()*s), &Expr::Val(F::from(5)));
}