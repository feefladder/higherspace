use std::ops::Mul;
use super::*;

impl Mul for Expr {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    todo!("HELPS!");
  }
}

// /// Multiplication for epressions.
// /// MAY mutate the input values, 
// impl Mul for Expr {
//   type Output = Self;
//   fn mul(mut self,mut rhs: Self) -> Self::Output {
//       match (&mut self, &mut rhs) {
//         (Expr::Val(vs), Expr::Val(vr)) => {
//           match (Rc::<F>::get_mut(vs), Rc::get_mut(vr)) {
//               (Some(v), _) => {
//                 // Mutate our own value and return self
//                 *v *= &**vr;
//                 return self;
//               },
//               (_, Some(v)) => {
//                 *v *= &**vs;
//                 rhs
//               },
//               (None, None) => {
//                 println!("Copying value!");
//                 Expr::Val(Rc::new(&**vs * &**vr))
//               }
//             }
//         },
//         (Expr::Val(vs), Expr::Const(cr)) => {
//           Expr::Sum(
//             Rc::new(
//               Sum{
//                 terms: vec![
//                   (
//                     **vs,
//                     Expr::Const(Rc::clone(cr))
//                   ),
//                 ]
//               }
//             )
//           )
//         },
//         (Expr::Const(_), Expr::Val(_)) => rhs*self,
//         (Expr::Val(vs), Expr::Sum(sum_r)) => {
//           match Rc::get_mut(sum_r) {
//             Some(s) => {
//               s.terms.iter_mut().for_each(|(c, e)| *c *= **vs);
//               rhs
//             },
//             None => {
//               Expr::Sum(
//                 Rc::new(
//                   Sum{
//                     terms: sum_r.terms.iter().map(|(c,e)| (*c*(**vs),*e))
//                     .collect()
//                   }
//                 )
//               )
//             }
//           }
//         },
//         (Expr::Sum(_),Expr::Val(_)) => rhs*self,
//         (Expr::Val(vs), _) => {
//           Expr::Sum(
//             Rc::new(
//               Sum{
//                 terms: vec![(**vs, rhs)]
//               }
//             )
//           )
//         },
//         (_, Expr::Val(_)) => rhs*self,
//         (Expr::Const(cs),Expr::Const(cr)) => {
//           if cs == cr {
//             Expr::Prod(Rc::new(
//               Prod {factors: vec![(&self, F::from(2))]}
//             ))
//           } else {
//             Expr::Prod(Rc::new(
//               Prod { factors: vec![(&self, F::from(1)),(rhs, F::from(1))] }
//             ))
//           }
//         },
//         (Expr::Const(cs), Expr::Sum(sr)) => {
//           match Rc::<Sum>::get_mut(sr) {
//               Some(sum_r) => {
//                 sum_r.terms.iter_mut().for_each(|(_, ex)| *ex = self * *ex);
//                 rhs
//               },
//               None => {
//                 Expr::Sum(
//                   Rc::new(
//                     Sum { 
//                       terms: sr.terms.iter().map(|(c, ex)| (*c, self* *ex)).collect()
//                      }
//                   )
//                 )
//               }
//           }
//         }
//         _ => {
//           todo!("implement multiplication for other types")
//         }
//       }
//   }
// }
