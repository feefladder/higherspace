use fraction::error::ParseError;
use lyn::{Scanner as Sc};
use std::{
  ops::{Deref, DerefMut},
  collections::HashMap, str::FromStr,
};
use ordered_float::NotNan;
use crate::{
  io_traits::Char,
  expr_field::{Expr, ExprFn, structs::{Const, sum::sort_svec}, FieldTrait}, F
};
pub struct Scanner<'a, Field: FieldTrait<'a>>{
  inner: Sc,
  c_map: HashMap<char, Const>,
  field: &'a Field,
}

#[derive(Debug)]
pub enum Error<'a, Field: FieldTrait<'a>> {
  EmptyExpr,
  Character(usize, Expr<'a, Field>)
}

impl<'a, Field: FieldTrait<'a>> Scanner<'a, Field> {
  pub fn new(string: &str, field: &'a Field) -> Self {
    Scanner { inner: Sc::new(string), c_map: Scanner::<Field>::c_map(), field }
  }

  pub fn c_map() -> HashMap<char, Const> {
    let mut m = HashMap::new();
    let v = vec![
      Const {
        ch: 'π',
        ascii: "pi",
        f64: NotNan::new(std::f64::consts::PI).unwrap(),
      },
      Const {
        ch: 'e',
        ascii: "e",
        f64: NotNan::new(std::f64::consts::E).unwrap(),
      }
    ];
    for c in v {
      m.insert(c.ch(), c);
    }
    m
  }
}

/// Scans while fn_match returns `true`
/// ```rust
/// use exact::expr_field::{
///   structs::type_field::TypeField,
///   io::parse_display::{Scanner, scan}
/// };
/// let f = TypeField::default();
/// let mut sc = Scanner::new("123456789)))).", &f);
/// assert_eq!(&scan(&mut sc, |c| c!=Some(&')')),"123456789");
/// assert_eq!(&scan(&mut sc, |c| c==Some(&')')),"))))");
/// // assert_eq!(sc.inner.peek(), &'.');
/// ```
pub fn scan<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>, fn_match: impl Fn(Option<&char>) -> bool ) -> String {
  let mut s = String::new();
  while fn_match(sc.inner.peek()) && sc.inner.peek() != None {
    s.push(*sc.inner.pop().unwrap())
  }
  s
}

fn err_or_eq<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>, c: &char) -> Result<(), ()> {
  if !sc.inner.take(c) {
    eprintln!("Did not find {} at position: {}, found: {}", c, sc.inner.cursor(), sc.inner.peek().unwrap_or(&'\0'));
    Err(())
  } else {
    Ok(())
  }
}

pub fn pd_expr<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Expr<'a, Field>{
  let cha = sc.inner.peek();
  if cha == None {
    return Expr::Zero(sc.field);
  }
  match *cha.unwrap() {
    'O' => {sc.inner.pop();Expr::Zero(sc.field)},
    'I' => {sc.inner.pop();Expr::One(sc.field)},
    '?' => {sc.inner.pop();Expr::InDet(sc.field)},
    '∞' => {sc.inner.pop();Expr::Infty(sc.field, fraction::Sign::Plus)},
    // Value with optional 'ξ'
    'ξ' => {
      sc.inner.pop();
      sc.field.add_val(pd_frac(sc).unwrap())
    },
    // value without 'ξ' OR "-∞"
    c if "-/0123456789".contains(c) => {
      sc.field.add_val(pd_frac(sc).unwrap())
    }
    // Sum
    'Σ' => {
      sc.inner.pop();
      pd_sum(sc)
    },
    // Prod
    'Π' => {
      sc.inner.pop();
      pd_prod(sc).unwrap()
    },
    // Fns
    '√' => {sc.inner.pop();sc.field.add_fn(ExprFn::Sqrt(pd_expr(sc)))},
    '🕢' => {sc.inner.pop();sc.field.add_fn(ExprFn::Sin(pd_expr(sc)))},
    '🕑' => {sc.inner.pop();sc.field.add_fn(ExprFn::Cos(pd_expr(sc)))},
    '🕘' => {sc.inner.pop();sc.field.add_fn(ExprFn::Tan(pd_expr(sc)))},
    _ => {pd_const(sc)},
  }
}

fn pd_frac<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Result<F, ParseError> {
  let s = scan(sc, |oc| match oc {
    None => false,
    Some(c) => {
      "-/0123456789∞".contains(*c)
    }
  });
  if s.contains('∞') {
    if s.chars().nth(0) == Some('-') {
      Ok(F::neg_infinity())
    } else {
      Ok(F::infinity())
    }
  } else {
    F::from_str(&s)
  }
}

fn pd_const<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Expr<'a, Field> {
  let c = *sc.inner.pop().unwrap();
  if sc.c_map.contains_key(&c) {
    sc.field.add_const(sc.c_map[&c])
  } else {
    match c {
      _ => {
        err_or_eq(sc, &'(');
        sc.field.add_const(Const {
          ch: c,
          ascii: "",
          f64: NotNan::from_str(
            &scan(
              sc,
              |ch| ch != Some(&')')
            )
          ).unwrap()
        })
      }
    }
  }
}

fn pd_sum<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Expr<'a, Field> {
  // [(1/2,I),(1/2,√5)]
  // ^        
  if sc.inner.take(&'[') {
    let mut v: Vec<(F, Expr<'a, Field>)> = Vec::new();
    // scan until closing bracket
    // (1/2,I),(1/2,√5))
    //  ^        ^      ^
    while sc.inner.peek().unwrap() != &']' {
      // (1/2,I),(1/2,√5))
      // ^------^ ^------^
      let t = pd_sum_tuple(sc).unwrap();
      v.push(t);
      // (1/2,I),(1/2,√5))
      //         ^  no err^
      if sc.inner.take(&',') {
      }
    }
    // [(1/2,I),(1/2,√5)]
    //                   ^
    err_or_eq(sc, &']');


    sort_svec(&mut v);
    sc.field.add_svec(v)
  } else {
    let t = pd_sum_tuple(sc).unwrap();
    sc.field.add_svec(vec![t])
  }
}

fn pd_sum_tuple<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Result<(F, Expr<'a, Field>), Error<'a, Field>> {
  err_or_eq(sc, &'(');
  let f: F = pd_frac(sc).unwrap();
  // (1/2,I),(1/2,√5)
  //     ^        ^
  err_or_eq(sc, &',');
  let e: Expr<'a, Field> = pd_expr(sc);
  // (1/2,I),(1/2,√5)
  //        ^        ^
  err_or_eq(sc, &')');
  Ok((f,e))
}


/// Prod Π[(I,1/2),(√5,1/2)] or Π(I,1/2) or Π{_[n=]<int>}{[^]<int>}[<expr>]
/// will match the part without 'Π': [(I,1/2),(√5,1/2)]
/// (expr,frac),
fn pd_prod<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Result<Expr<'a, Field>, Error<'a, Field>> {
  // [(I,1/2),(√5,1/2)]
  // ^        
  if sc.inner.take(&'[') {
    // err_or_eq(sc, &'[')?;
    let mut v = Vec::new();
    // scan until closing bracket
    // (I,1/2),(√5,1/2))
    //  ^        ^      ^
    while sc.inner.peek().unwrap() != &']' {
      // (I,1/2),(√5,1/2)
      // ^------^ ^------^
      let t = pd_prod_tuple(sc)?;
      v.push(t);
      // (I,1/2),(√5,1/2))
      //         ^  no err^
      if sc.inner.take(&',') {
      } else {
      };
    }
    // [(I,1/2),(√5,1/2)]
    //                   ^
    err_or_eq(sc, &']');

    v.sort_unstable();

    Ok(sc.field.add_pvec(v))
  } else {
    let t = pd_prod_tuple(sc)?;
    Ok(sc.field.add_pvec(vec![t]))
  }
}

fn pd_prod_tuple<'a, Field: FieldTrait<'a>>(sc: &mut Scanner<'a, Field>) -> Result<(Expr<'a, Field>, F), Error<'a, Field>> {
  err_or_eq(sc, &'(');
  let e: Expr<'a, Field> = pd_expr(sc);
  // (I,1/2),(√5,1/2)
  //    ^        ^
  err_or_eq(sc, &',');
  let f: F = pd_frac(sc).unwrap();
  // (I,1/2),(√5,1/2)
  //        ^        ^
  err_or_eq(sc, &')');
  Ok((e,f))
}


#[cfg(test)]
mod parsed_tests {
  use std::ops::Not;

use super::*;
  // use crate::expr_rc::Expr;
  use fraction::One;
use num_traits::Zero;
  use crate::expr_field::structs::{type_field::TypeField, Prod};
  // use std::f64::consts::PI;
  // let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
  // let E: Const = Const{ ch: 'e', ascii: "e"  , f64: NotNan::new(std::f64::consts::E ).unwrap() };
  // let PHI: Const = Const{ ch: 'φ', ascii: "phi", f64: NotNan::from_str("1.618").unwrap()};
  #[test]
  fn test_pd_frac() {
    let test_vec: Vec<(&str, F)> = vec![
      ("1",F::one()),
      ("-1",-F::one()),
      ("1/2",F::new(1u8,2u8)),
      ("-1/2",-F::new(1u8,2u8)),
    ];
    let f = TypeField::default();
    for (asdf, res) in test_vec {
      assert_eq!(pd_frac(&mut Scanner::new(asdf, &f)), Ok(res))
    }
  }

  #[test]
  fn test_parse_fundamentals() {
    let f = TypeField::default();
    let tv = vec![
      ("O",Expr::Zero(&f)),
      ("I",Expr::One(&f)),
      ("?",Expr::InDet(&f)),
      ("∞",Expr::Infty(&f,fraction::Sign::Plus)),
      // ("-∞",Expr::Infty(&f,fraction::Sign::Minus)),
    ];
    for (t,res) in tv {
      assert_eq!(f.parse(t),res);
      assert_eq!(t,format!("{}",res));
    }
  }

  #[test]
  fn test_parse_val() {
    let test_vec = vec![
      ("2", F::from(2)),
      ("5/2",F::new(5u8,2u8)),
    ];
    let f = TypeField::default();
    for (asdf, res) in test_vec {
      let v = f.add_val(res);
      assert_eq!(f.parse(asdf), v);

      assert_eq!(asdf, format!("{}", v));
    }
  }
  #[test]
  fn test_parse_const() {
    // cosnt
    let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    let c_e: Const = Const{ ch: 'e', ascii: "e"  , f64: NotNan::new(std::f64::consts::E ).unwrap() };
    let c_phi: Const = Const{ ch: 'φ', ascii: "phi", f64: NotNan::from_str("1.618").unwrap()};
    let test_vec = vec![
      ("π",c_pi),
      ("e",c_e),
      ("φ(1.618)",c_phi),
    ];
    let f = TypeField::default();
    for (asdf, res) in test_vec {
      let c = f.add_const(res);
      assert_eq!(f.parse(asdf), c)
    }
  }
  #[test]
  fn test_parse_sum_single() {
    // sum single
    let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    // let E: Const = Const{ ch: 'e', ascii: "e"  , f64: NotNan::new(std::f64::consts::E ).unwrap() };
    // let PHI: Const = Const{ ch: 'φ', ascii: "phi", f64: NotNan::from_str("1.618").unwrap()};
    let f = TypeField::default();
    let pi = f.add_const(c_pi);
    let sqrt5 = f.add_fn(ExprFn::Sqrt(f.add_val(F::from(5))));
    let tv = vec![
      // ("Σ(1/2,√ξ5)", vec![(F::new(1u8, 2u8),sqrt5)]),
      ("Σ(1/2,√5)", vec![(F::new(1u8, 2u8),sqrt5)]),
      ("Σ(2,π)", vec![(F::from(2), pi)]),
      // ("Σ[(2,π)]", vec![(F::from(2), pi)]),
    ];
    for (t, mut res) in tv {
      println!("{} =?", t);
      let s = f.add_svec(res);
      assert_eq!(f.parse(t), s);
      assert_eq!(t,format!("{}",s));
    }
    // f.parse("Σ(2,π)");
    // f.parse("Σ(1/2,√ξ5)");
  }
  #[test]
  fn test_parse_sum_multi() {
    let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    // let E: Const = Const{ ch: 'e', ascii: "e"  , f64: NotNan::new(std::f64::consts::E ).unwrap() };
    // let PHI: Const = Const{ ch: 'φ', ascii: "phi", f64: NotNan::from_str("1.618").unwrap()};
    let f = TypeField::default();
    let pi = f.add_const(c_pi);
    let one = Expr::One(&f);
    let sqrt5 = f.add_fn(ExprFn::Sqrt(f.add_val(F::from(5))));
    let tv = vec![
      ("Σ[(1/2,I),(1/2,√5)]", vec![(F::new(1u8, 2u8),one),(F::new(1u8,2u8),sqrt5)]),
      ("Σ[(2,I),(1,π)]", vec![(F::from(2), one), (F::one(), pi)]),
      ("Σ[(1/2,√5),(1/2,I)]", vec![(F::new(1u8,2u8),one),(F::new(1u8,2u8),sqrt5)]),
    ];
    // sum multi
    for (t, mut res) in tv {
      println!("{} =? {:?}", f.parse(t), res);
      sort_svec(&mut res);
      println!("{} =? {:?}", f.parse(t), res);
      let s = f.add_svec(res);
      println!("{} =? {:?}", f.parse(t), s);
      assert_eq!(f.parse(t), s);
      // assert_eq!(t, format!("{}",s));
    }
  }
  #[test]
  fn test_parse_prod_single() {
    // prod single
    let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    let f = TypeField::default();
    let pi = f.add_const(c_pi);
    let tv = vec![
      ("Π(π,2)", vec![(pi, F::from(2))]),
      // ("Π[(π,2)]", vec![(pi, F::from(2))]),
    ];
    for (t, mut res) in tv {
      res.sort_unstable();
      let p = f.add_pvec(res);
      println!("{} =? {}", t, p);
      assert_eq!(f.parse(t), p);
      assert_eq!(t, format!("{}", p));
    }
  }
  #[test]
  fn test_parse_prod_multi() {
    // prod multi
    // let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    let f = TypeField::default();
    let pi = f.parse("π");
    let sqrt5 = f.parse("√5");//f.add_fn(ExprFn::Sqrt(f.add_val(F::from(5))));
    let tv = vec![
      // ("Π[(√5,1),(π,1)]", vec![(pi, F::from(1)),(sqrt5,F::from(1))]),
      ("Π[(π,1),(√5,1)]", vec![(pi, F::from(1)),(sqrt5,F::from(1))]),
    ];
    for (t, mut res) in tv {
      println!("{}->",Prod{factors: res.clone()});
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      println!("{:?}",pi.cmp(&sqrt5));
      let rt = res.clone();
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      println!("{:?}",pi.cmp(&sqrt5));
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      println!("{:?}, {:?}",pi.cmp(&sqrt5),pi<sqrt5);
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      println!("{:?}",pi.cmp(&sqrt5));
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      res.sort();
      // println!("{:?}<{:?}:{} ==:{} >:{}",res[0],res[1],res[0]<res[1],res[0]==res[1],res[0]>res[1]);
      println!("{:?} vs {:?} : {:?}",res[0].0,res[1].0,res[0].0.cmp(&res[1].0));
      assert_eq!(res, rt);
      let p = f.add_pvec(res);
      println!("{} =? {}", t, p);
      println!("{} =? {}", f.parse(t), p);
      assert_eq!(f.parse(t), p);
      // assert_eq!(t,format!("{}", p));
    }
    // assert_eq!(Expr::parse_display(.to_string()),Ok(Expr::prod_pi_times_sqrt_i(5)));
  }
  #[test]
  fn test_parse_sqrt() {
    // sqrt
    let c_pi: Const = Const { ch: 'π', ascii: "pi", f64: NotNan::new(std::f64::consts::PI).unwrap() };
    let f = TypeField::default();
    let pi = f.add_const(c_pi);
    // let sqrt5 = f.add_fn(ExprFn::Sqrt(f.add_val(F::from(5))));
    let tv = vec![
      ("√5", f.add_val(F::from(5))),
      ("√5/2", f.add_val(F::new(5u8,2u8))),
      ("√π", pi),
      ("√Σ(2,π)", f.add_svec(vec![(F::from(2),pi)])),
      // ("√Σ[(2,π)]", f.add_svec(vec![(F::from(2),pi)])),
      ("√Π(π,2)", f.add_pvec(vec![(pi,F::from(2))])),
    ];
    for (t, res) in tv {
      println!("{} =?", t);
      assert_eq!(f.parse(t), f.add_fn(ExprFn::Sqrt(res)));
      assert_eq!(t,format!("{}", f.add_fn(ExprFn::Sqrt(res))));
    }
  }
}