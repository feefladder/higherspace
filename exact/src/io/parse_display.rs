use std::{
  rc::Rc,
  str::FromStr,
};

/// Parser for parsing display-type strings
/// e.g. 
use lyn::Error;

use crate::{
  F,
  expr_rc::Expr,
  structs::Const,
  io::{
    scanner::Scanner,
    traits::ParseDisplay,
  },
};


impl ParseDisplay for Expr {
  fn parse_display(input: String) -> Result<Expr, Error> {
    let mut sc = Scanner::new(&input.replace(" ", ""));
    pd_expr(&mut sc)
  }
}

/// Σ((1/2,ξ1),(1/2,√5)) -> Expr(1/2+1/2√(5))
fn pd_expr(sc: &mut Scanner) -> Result<Expr, Error> {
  let cha = sc.pop();
  if cha == None {
    return Err(Error::EndOfLine);
  }
  // dereference-unwrap to not have &mut sc anymore for
  // c:char => pd_const(c)
  // branch
  // If we had a reference to cha, we could mutate it in theory
  match *cha.unwrap() {
    // Some(c) => {
    //   match c {
        'ξ' => {
          let f: F = pd_frac(sc)?;
          Ok(Expr::from(f))
        },
        'Σ' => pd_sum(sc),
        'Π' => pd_prod(sc),
        '√' => pd_sqrt(sc),
        c => pd_const(sc, c),
    //   }
    // }
  }
}

fn pd_const(sc: &mut Scanner, c: char) -> Result<Expr, Error> {
  if sc.has(&c) {
    Ok(Expr::Const(Rc::new(sc.get_const(&c))))
  } else {
    sc.err_or_eq(&'(')?;
    // TODO: actually parse f64
    let v = pd_num::<f64>(sc, 10)?;
    sc.err_or_eq(&')')?;
    Ok(Expr::Const(Rc::new(Const { ch: c, f64: v })))
  }
}

fn pd_sqrt(sc: &mut Scanner) -> Result<Expr, Error> {
  if sc.peek().unwrap_or(&'x').is_digit(10) {
    // √5
    let f: F = pd_frac(sc)?;
    Ok(Expr::sqrt_frac(f))
  } else {
    // √expr
    let e: Expr = pd_expr(sc)?;
    Ok(Expr::sqrt_expr(e))
  }
}

/// Fraction: [-]<num>/<den>
/// where
fn pd_frac(sc: &mut Scanner) -> Result<F, Error> {
  let mut f = F::from(1);
  if sc.take(&'-') {
    f *= -1;
  }
  f *= pd_num::<u32>(sc, 10)?;
  if sc.take(&'/') {
    f /= pd_num::<u32>(sc, 10)?;
  }
  Ok(f)
}

/// int: sequence of digits (0-radix)
/// May be nice to allow for hex etc e.g. also accept pre-indicators and use to set radix
/// But I'm lazy :P
fn pd_num<T: FromStr>(sc: &mut Scanner, radix: u32) -> Result<T, Error> {
  // default char signaling string end
  let term = '\0';

  let mut s = String::new();
  let mut c = *sc.peek().unwrap_or(&term);
  
  while (c.is_digit(radix) || c == '.') && c != term {
      s.push(*sc.pop().unwrap());
      c = *sc.peek().unwrap_or(&term);
  }
  // match to get our error type
  match s.parse::<T>() {
    Ok(v) => Ok(v),
    Err(_) => Err(Error::Character(sc.cursor())),
  }
}

/// Sum: Σ[(1/2,ξ1),(1/2,√5)] or Σ(1/2,ξ1)
/// will match the inner part: [(1/2,ξ1),(1/2,√5)]
/// (frac,expr)
fn pd_sum(sc: &mut Scanner) -> Result<Expr, Error> {
  // [(1/2,ξ1),(1/2,√5)]
  // ^        
  if sc.take(&'[') {
    let mut v: Vec<(F, Expr)> = Vec::new();
    // scan until closing bracket
    // (1/2,ξ1),(1/2,√5))
    //  ^        ^      ^
    while sc.peek().unwrap() != &']' {
      // (1/2,ξ1),(1/2,√5))
      // ^------^ ^------^
      let t = pd_sum_tuple(sc)?;
      v.push(t);
      // (1/2,ξ1),(1/2,√5))
      //         ^  no err^
      if sc.take(&',') {
      }
    }
    // [(1/2,ξ1),(1/2,√5)]
    //                   ^
    sc.err_or_eq(&']')?;
    Ok(Expr::from(v))
  } else {
    let t = pd_sum_tuple(sc)?;
    Ok(Expr::from(vec![t]))
  }
}


fn pd_sum_tuple(sc: &mut Scanner) -> Result<(F, Expr), Error> {
  sc.err_or_eq(&'(')?;
  let f: F = pd_frac(sc)?;
  // (1/2,ξ1),(1/2,√5)
  //     ^        ^
  sc.err_or_eq(&',')?;
  let e: Expr = pd_expr(sc)?;
  // (1/2,ξ1),(1/2,√5)
  //        ^        ^
  sc.err_or_eq(&')')?;
  Ok((f,e))
}

/// Prod Π[(ξ1,1/2),(√5,1/2)] or Π(ξ1,1/2)
/// will match the part without 'Π': [(ξ1,1/2),(√5,1/2)]
/// (expr,frac),
fn pd_prod(sc: &mut Scanner) -> Result<Expr, Error> {
  // [(ξ1,1/2),(√5,1/2)]
  // ^        
  if sc.take(&'[') {
    // sc.err_or_eq(&'[')?;
    let mut v: Vec<(Expr,F)> = Vec::new();
    // scan until closing bracket
    // (ξ1,1/2),(√5,1/2))
    //  ^        ^      ^
    while sc.peek().unwrap() != &']' {
      // (ξ1,1/2),(√5,1/2)
      // ^------^ ^------^
      let t = pd_prod_tuple(sc)?;
      v.push(t);
      // (ξ1,1/2),(√5,1/2))
      //         ^  no err^
      if sc.take(&',') {
      } else {
      };
    }
    // [(ξ1,1/2),(√5,1/2)]
    //                   ^
    sc.err_or_eq(&']')?;
    Ok(Expr::from(v))
  } else {
    let t = pd_prod_tuple(sc)?;
    Ok(Expr::from(vec![t]))
  }
}

fn pd_prod_tuple(sc: &mut Scanner) -> Result<(Expr, F), Error> {
  sc.err_or_eq(&'(')?;
  let e: Expr = pd_expr(sc)?;
  // (ξ1,1/2),(√5,1/2)
  //    ^        ^
  sc.err_or_eq(&',')?;
  let f: F = pd_frac(sc)?;
  // (ξ1,1/2),(√5,1/2)
  //        ^        ^
  sc.err_or_eq(&')')?;
  Ok((e,f))
}

#[cfg(test)]
mod parsed_tests {
  use super::*;
  use crate::expr_rc::Expr;
  use fraction::One;
  // use std::f64::consts::PI;

  #[test]
  fn test_pd_frac() {
    assert_eq!(pd_frac(&mut Scanner::new("1")),Ok(F::one()));
    assert_eq!(pd_frac(&mut Scanner::new("-1")),Ok(-F::one()));
    assert_eq!(pd_frac(&mut Scanner::new("1/2")),Ok(F::new(1u8,2u8)));
    assert_eq!(pd_frac(&mut Scanner::new("-1/2")),Ok(-F::new(1u8,2u8)));
  }

  #[test]
  fn test_parse_val() {
    // val
    assert_eq!(Expr::parse_display("ξ1".to_string()), Ok(Expr::from(1u32)));
    assert_eq!(Expr::parse_display("ξ2".to_string()),Ok(Expr::val_i(2)));
    assert_eq!(Expr::parse_display("ξ5/2".to_string()),Ok(Expr::val_frac(F::new(5u8,2u8))));
  }
  #[test]
  fn test_parse_const() {
    // cosnt
    assert_eq!(Expr::parse_display("π".to_string()),Ok(Expr::c_pi()));
    assert_eq!(Expr::parse_display("e".to_string()),Ok(Expr::c_e()));
    assert_eq!(Expr::parse_display("φ(1.618)".to_string()),Ok(Expr::from(('φ',1.618))));
  }
  #[test]
  fn test_parse_sum_single() {
    // sum single
    assert_eq!(Expr::parse_display("Σ(1/2,√5)".to_string()  ), Ok(Expr::from(vec![(F::new(1u8, 2u8),Expr::sqrt_i(5))])));
    assert_eq!(Expr::parse_display("Σ[(1/2,√5)]".to_string()), Ok(Expr::from(vec![(F::new(1u8, 2u8),Expr::sqrt_i(5))])));
    assert_eq!(Expr::parse_display("Σ(2,π)".to_string())  ,Ok(Expr::sum_i_pi(2)));
    assert_eq!(Expr::parse_display("Σ[(2,π)]".to_string()),Ok(Expr::sum_i_pi(2)));
  }
  #[test]
  fn test_parse_sum_multi() {
    // sum multi
    assert_eq!(Expr::parse_display("Σ[(1/2,ξ1),(1/2,√5)]".to_string()), Ok(Expr::sum_phi()));
    assert_eq!(Expr::parse_display("Σ[(2,ξ1),(1,π)]".to_string()),Ok(Expr::sum_i_plus_pi(2)));
    assert_eq!(Expr::parse_display("Σ[(1/2,ξ1),(1/2,√5)]".to_string()),Ok(Expr::sum_phi()));
  }
  #[test]
  fn test_parse_prod_single() {
    // prod single
    assert_eq!(Expr::parse_display("Π(π,2)".to_string()  ),Ok(Expr::prod_pi_i(2)));
    assert_eq!(Expr::parse_display("Π[(π,2)]".to_string()),Ok(Expr::prod_pi_i(2)));
  }
  #[test]
  fn test_parse_prod_multi() {
    // prod multi
    assert_eq!(Expr::parse_display("Π[(π,1),(√5,1)]".to_string()),Ok(Expr::prod_pi_times_sqrt_i(5)));
  }
  #[test]
  fn test_parse_sqrt() {
    // sqrt
    assert_eq!(Expr::parse_display("√5".to_string()),Ok(Expr::sqrt_i(5)));
    assert_eq!(Expr::parse_display("√5/2".to_string()),Ok(Expr::sqrt_frac(F::new(5u8,2u8))));
    assert_eq!(Expr::parse_display("√π".to_string()),Ok(Expr::sqrt_expr(Expr::c_pi())));
    
    assert_eq!(Expr::parse_display("√Σ(2,π)".to_string()  ),Ok(Expr::sqrt_expr(Expr::sum_i_pi(2))));
    assert_eq!(Expr::parse_display("√Σ[(2,π)]".to_string()),Ok(Expr::sqrt_expr(Expr::sum_i_pi(2))));
  }
}