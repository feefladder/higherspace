pub use num_traits::Inv;


use crate::{
  F,
  One,
  expr_rc::Expr,
};

impl Inv for Expr {
  type Output = Expr;
  fn inv(self) -> Self::Output {
      match &self {
        Expr::Val(v) => {
          let f: F = **v;
          Expr::from(F::one()/f)
        },
        Expr::Prod(p) => {
          todo!()
        }
        _ => Expr::from(vec![(self, F::from(-1))])
      }
  }
}

#[cfg(test)]
mod test_inv
{
    use crate::expr_rc::{
      Inv,
      Expr,
    };

  #[test]
  fn test_inv() {
    let test_vec: Vec<(&str, &str)> = vec![
      ("ξ1","ξ1"),
      ("ξ2","ξ1/2"),
      ("ξ-2","ξ-1/2"),
      ("π","Π(π,-1)"),
      ("Σ[(1,ξ1),(1,π)]","Π(Σ[(1,ξ1),(1,π)],-1)"),
      ("Π(π,2)","Π(π,-2)"),
      ("Π[(π,1),(e,1)]","Π[(π,-1),(e,-1)]"),
      ("√2","Σ(1/2,√2)"),

    ];
    for (a,b) in test_vec {
      let e_a = Expr::try_from(a).unwrap();
      let e_b = Expr::try_from(b).unwrap();
      assert_eq!(e_a.clone().inv(), e_b);
      assert_eq!(e_b.inv(), e_a);
    }
  }
}