use std::{
  cell::Ref,
  ops::{
  Add,
  Mul,
}};

use num_traits::Zero;

use crate::{
  F,
    expr_field::{
    Expr,
    FieldTrait,
    structs::Sum,
    simplify::sum::simplify_svec, SVec,
  }
};

#[inline]
pub fn sort_svec<A: Ord,B: Ord>(v: &mut Vec<(A,B)>) {
  v.sort_unstable_by(|(a1,b1),(a2,b2)| (b1,a1).cmp(&(b2,a2)))
}

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
//       simplify_svec([self.terms,(F::one(), rhs)])
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
      simplify_svec(
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
impl<'a, Field> Mul for Sum<'a, Field> where Expr<'a, Field>: Mul<Expr<'a, Field>, Output = Expr<'a, Field>> + Ord{
  type Output = SVec<'a, Field>;
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
    let mut res = simplify_svec(v_new);
    res.retain(|(c,_)| !c.is_zero());
    res
  }
}

// impl<'a, Field: 'a> Mul<F> for Ref<Sum<'a, Field>> where Expr<'a, Field>: Mul<Expr<'a, Field>, Output = Expr<'a, Field>> + Ord{
//   type Output = SVec<'a, Field>;
//   fn mul(self, rhs: F) -> Self::Output {
//     Sum { 
//     }
//   }
// }


impl<'a, Field: FieldTrait<'a>> Eq for Sum<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> PartialEq for Sum<'a, Field> {
  #[inline(always)]
  fn eq(&self, other: &Sum<'a, Field>) -> bool {
    self.terms == other.terms
    // self.terms.unordered_eq(&other.terms)
    // todo!()
  }
}

impl<'a, Field: FieldTrait<'a>> PartialOrd for Sum<'a, Field> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
/// We actually want to overwrite Ord for the tuple (F, Expr)
/// so that it first compares Expr and then F if Expr1==Expr2
impl<'a, Field: FieldTrait<'a>> Ord for Sum<'a, Field> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.terms.len() == other.terms.len() {
      self.terms.iter()
        .map(|(a,b)| (b,a))
        .cmp(
          other.terms.iter().map(|(a,b)| (b,a))
        )
    } else {
      self.terms.len().cmp(&other.terms.len())
    }
  }
}

#[cfg(test)]
mod test_sum {
    use crate::{F,One,expr_field::{structs::type_field::TypeField, FieldTrait}};

    use super::sort_svec;


  #[test]
  fn test_sort_svec() {
    let f = TypeField::default();
    let tv = vec![
      (vec![(F::one(),f.parse("π")),(F::one(),f.parse("I"))],vec![(F::one(),f.parse("I")),(F::one(),f.parse("π"))]),
    ];
    for (mut a,b) in tv {
      sort_svec(&mut a);
      assert_eq!(a,b);
    }
  }
}