/// rules
/// Sum only: Set values co-things to ξ1
/// Σ[(2,ξ5),(1,ξ2)] -> Σ[(10,ξ1),(2,ξ1)]
/// Π(ξ2,5/2) -> Π[(ξ4,1),(ξ2,1/2)]
/// non-fractional Prod only:
/// Π[(ξ2,2),(ξ3,2)] -> Π[(ξ4,1),(ξ9,1)]
/// fractional powers:
/// always collect as many factors as possible under the same power:
/// Π[(π,1/3),(e,1/3)] -> Π(Π[(π,1),(e,1)],1/3)
/// Π(Π[(π,1),(e,1)],1/3),(e,1)) -> Π[(π,1/3),(e,4/3)]
/// Square roots:
/// Π(π,3/2) -> Π[(π,1),(√π,1)]
/// π^(p/2) -> π^(p-1)√π
/// 
/// 
/// π^(p/q)√π -> π^(2p+q/2q)
/// Prod+Sum: Collect like terms/factors:
/// Π[(π,a),(π,b)] -> Π(π,a+b)
/// Σ[(α,π),(β,π)] -> Σ(α+β,π)
/// Prod+Sum: strip zeroes
/// Π(π,0) -> Π() -> ξ1
/// Σ(0,π) -> Σ() -> ξ1
/// Π[(π,0),(ξ5,1)] -> Π(ξ5,1)
/// Σ[(0,π),(5,ξ1)] -> Σ(5,ξ1)
/// 
/// Prod: filter out values: 
/// Π[(ξ5,1),(π,1)] -> Σ(5,Π(π,2)) // <- What happens if this is inside a sum?
/// 
/// 

///
/// ("Π(π,1/4)","√π","Π(π,3/4)"),
/// 
/// ("Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
/// ("Σ(2,√5)","√5","Σ(10,ξ1)"),
/// ("Σ(2,√5)","√5","ξ10"),

/// 
/// ("Π[(π,1),(√5,1)]","√5","Σ(5,π)"),
/// "Π(π,1)" -> π Done!
/// "Σ(10,ξ1)" -> "ξ10" Done?
/// ("Π[(π,2),(√5,1)]","√5","Σ(5,Π(π,2))"),


/// 
/// ("√Σ(2,π)","√π","Π[(π,1),(√2,1)]"),
/// ("√Σ(6,π)","√2","Σ(2,√Σ(3,π))"),
/// ("√Σ(2,π)","√2","Σ(2,√π)"),
/// ("Π(π,1/4)","√π","Π(π,3/4)"),
/// ("Π(π,2)","√π","Π[(π,2),(√π,1)]"),
/// ("π","√π", "Π[(π,1),(√π,1)]"),

// use super::Expr;

// impl Expr {
//   fn drop_zeroes() {

//   }
// }

pub mod prod;
pub mod sum;

#[inline]
pub fn collect_like<T: Clone + Copy>(vec: Vec<T>, fn_match: impl Fn(T, T) -> bool, mut fn_add: impl FnMut(&mut T,T)) -> Vec<T> {
  let mut new_vec: Vec<T> = Vec::new();
  for item in vec {
    match new_vec.iter_mut().find(|a|fn_match(**a,item)) {
      Some(matc) => {
        fn_add(matc, item);
      },
      None => {
        new_vec.push(item);
      }
    }
  }
  new_vec
}