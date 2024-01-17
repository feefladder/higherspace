use std::{ops::{
  Add,
  Mul,
}, collections::HashSet};
// use std::rc::Rc;

use core::hash::Hash;

use num_traits::Zero;

use crate::expr_field::{
  FieldTrait,
  structs::Sum,
  simplify::sum::collect_like_terms,
};

// /// Add a Const to this sum. Works on the vector.
// /// Does not check which field the Const belongs to
// /// ```raw
// /// Σ(2,√5) + π -> Σ[(2,√5),(1,π)] -> ..
// /// Σ(2,π)  + π -> Σ[(2,π),(1,π)]  -> Σ(3,π)
// /// ```
// impl<'a, ExType: ExprTrait<'a, > Add<Const> for Sum<ExType> {
//   type Output = Self;
//   fn add(self, rhs: Const) -> Self::Output {
//     Sum{ terms:
//       collect_like_terms([self.terms,(F::one(), rhs)])
//     }
//   }
// }

// impl<'a, Field: FieldTrait<'a>> Hash for Sum<'a, Field> {
//   fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//     // self.terms.sort_unstable_by()
//   }
// }

/// Add two sums. Works on the vectors.
/// ```raw
/// Σ(2,π)          + Σ(2,π)  -> Σ[(2,π),       (2,π)]        -> Σ(4,π),
/// Σ[(1,Ι),(1,√5)] + Σ(2,√5) -> Σ[(1,I),(1,√5),(2,√5)]       -> Σ[(1,I),(3,√5)]
/// Σ[(1,Ι),(1,√5)] "*2"      -> Σ[(1,I),(1,√5),(1,I),(1,√5)] -> Σ[(2,I),(2,√5)]
/// ```
/// ```
/// let f = FlatField::new();
/// //     add π to field-\
/// assert_eq!(f.parse("Σ(2,π)")+f.parse("Σ(2,π)"),f.parse("Σ(4,π)"))
/// // add Σ(2,π) to field-/
/// // Σ(4,π) is added to field by Expr-sum
/// assert_eq!(f,vec!["π","Σ(2,π)","Σ(4,π)"])
/// ```
/// MAY return an empty sum: `Σ()` or value: `Σ(v,Ι)`
impl<'a, Field: FieldTrait<'a>> Add for Sum<'a, Field> {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Sum{ terms: 
      collect_like_terms(
        [self.terms, rhs.terms].concat()
      )
    }
  }
}

/// Multiply sums
/// works directly on the vectors. Adds "inner" products to field
/// ```raw
/// Σ(2,π)          * Σ(2,π)  = Σ(4,Π(π,2)),
/// Σ[(1,Ι),(1,√5)] * Σ(2,√5) = Σ[(2,√5),(20,Ι)]
/// Σ[(1,Ι),(1,√5)] ^2        = Σ[(2,√5),(6,Ι)]
/// ```
/// ```
/// let f = FlatField::new();
/// //     add π to field-\
/// assert_eq!(f.parse("Σ(2,π)")*f.parse("Σ(2,π)"),f.parse("Σ(4,Π(π,2))"))
/// // add Σ(2,π) to field-/                                    |
/// // Π(π,2) is added to field as "inner": π*π-----------------/
/// // Σ(4,Π(π,2)) is added by Expr-prod
/// assert_eq!(f,vec!["π","Σ(2,π)","Π(π,2)","Σ(4,Π(π,2))"])
/// ```
/// MAY return a value: `Σ(v,I)`
impl<'a, Field: FieldTrait<'a>> Mul for Sum<'a, Field> {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    let v_s = self.terms;
    let v_r = rhs.terms;
    let mut v_new = Vec::new();
    // Σ[(1,I),(1,√5)] ^2 -> Σ[(1,I),(1,√5),(1,√5),(5,I)]
    // Σ(2,π)           * Σ(2,π) -> Σ(4,Π(π,2))
    //    Π(π,2) gets added to the field-/
    for (f_s, e_s) in &v_s {
      for (f_r, e_r) in &v_r {
        // simplification and adding to field happens in
        // e_s * e_r
        v_new.push((f_s*f_r,(*e_s)*(*e_r)))
      }
    }
    // Σ[(1,I),(1,√5),(1,√5),(5,I)] -> Σ[(6,I),(2,√5)]
    let mut res = collect_like_terms(v_new);
    res.retain(|(c,_)| !c.is_zero());
    Sum { terms:res}
  }
}

impl<'a, Field: FieldTrait<'a>> PartialEq for Sum<'a, Field> {
  fn eq(&self, other: &Sum<'a, Field>) -> bool {
    let s: HashSet<_> = self.terms.iter().collect();
    let o: HashSet<_> = other.terms.iter().collect();
    s == o
    // self.terms.unordered_eq(&other.terms)
    // todo!()
  }
}