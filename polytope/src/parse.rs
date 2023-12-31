use core::fmt;
use std::fmt::Debug;
use crate::{CoxNode, CoxEdge, CoxGraph, CoxGroup};

use fraction::GenericFraction;
use petgraph::prelude::NodeIndex;
// use petgraph::data::Build;

use std::error::Error;
use std::clone::Clone;


/// Minimal parser based on the following BNF:
/// <tope> ::= <subtope> [<branch>]
/// <subtope> ::= <node> {<edge> <node>}
/// <node> ::= [a-z] | "(" <bla ")"
/// <complexNode> ::= "(" <bla> ")"
/// <bla> ::= <pNode> | <nNode>
/// <nNode> ::= "-" <pNode>
/// <pNode> ::= [a-z]+
/// <edge> ::= [0-9]+ | <rationalEdge>
/// <rationalEdge> ::= "(" [0-9]+ "/" [0-9]+ ")"
///
/// <branch> ::= <brSep> <brIndex> | <brSep> <brIndex> <edge> <brSep> <brIndex> | <brSep> <brIndex> <edgeNode>
/// <brSep> ::= " *" | "*"
/// <brIndex> ::= <pIndex> | <nIndex>
/// <nIndex> ::= "-" [a-z]
/// <pIndex> ::= [a-z]

// pub fn parse<T: Clone + fmt::Debug + From<i32>>(input: &str) -> Result<CoxNode<T>, ()>{
//     let mut scanner = Scanner::new(input);

//     loop {
//         if !tope(&mut scanner) {
//             break;
//         }
//     }

//     Ok(node)
// }



//---------------- From the tut
#[derive(fmt::Debug)]
pub struct SyntaxError {
  message: String,
  level: String,
}

impl SyntaxError {
  // fn new_lex_error(message: String) -> Self {
  //     SyntaxError {
  //         message,
  //         level: "Lex".to_string(),
  //     }
  // }

  fn new_parse_error(message: String) -> Self {
    SyntaxError {
      message,
      level: "Parse".to_string(),
    }
  }
}

impl fmt::Display for SyntaxError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} Error {}", self.level, self.message)
  }
}

impl Error for SyntaxError {}
//---------------------end

#[derive(Debug)]
pub struct Scanner{
  cur: usize,
  chars: Vec<char>,
  node_index: char,
}

impl Scanner {
  pub fn new(string: &str) -> Self {
    Self {
      cur: 0,
      chars: string.chars().collect(),
      node_index: 'a',
    }
  }

  pub fn parse_tope<T: Debug + Clone + From<f64>>(&mut self) -> Result<CoxGroup<T>, SyntaxError> {
    let mut graph = self.tope::<T>()?;
    if self.cur != self.chars.len() {
      return Err(SyntaxError::new_parse_error("Not at the end of the string!".to_string()));
    }
    // remove 2-edges from the graph
    graph.retain_edges(|g,i| g[i] != CoxEdge::Int(2));

    Ok(CoxGroup{ graph: graph, polys: Vec::new()})
  }

  /// Checks for a complete polytope
  /// e.g. x3o3o *b3o
  /// returns a graph
  fn tope<T: Debug + Clone+ From<f64>>(&mut self) -> Result<CoxGraph<T>, SyntaxError> {
    println!("Parsing tope! {:?}", self);
    let mut tope = self.subtope::<T>()?;
    println!("Parsing tope branch! {:?}", self);
    self.branch(&mut tope)?;
    Ok(tope)
  }
  
  /// Checks for a sub-polytope
  /// e.g. x4o3o3o
  /// returns a graph
  fn subtope<T: Debug + Clone + From<f64>>(&mut self) -> Result<CoxGraph<T>, SyntaxError> {
    let mut graph = CoxGraph::<T>::default();
    let node = self.node::<T>()?;
    // FIXME: ugly
    let n0: NodeIndex = graph.add_node(node);
    self.edge_nodes(&mut graph, n0)?;

    Ok(graph)
  }

  fn edge_nodes<T: Debug + Clone + From<f64>>(&mut self, tope: &mut CoxGraph<T>, mut node_index: NodeIndex) -> Result<(), SyntaxError>{
    loop {
      match self.peek() {
        Some(e) => {
          if !e.is_numeric() {
            break;
          }
        }
        None => {
          break;
        }
      }
      let edge = self.edge()?;
      let node = self.node()?;
      let n1 = tope.add_node(node);
      tope.add_edge(node_index, n1, edge);
      // FIXME: ugly
      node_index = n1;
    }
    Ok(())
  }

  /// checks for a node
  /// e.g. x, (-x), (f+f)
  /// returns a CoxNode
  fn node<T: Debug + Clone + From<f64>>(&mut self) -> Result<CoxNode<T>, SyntaxError>{
    match self.pop() {
      Some(c) => {
        match c {
          '(' => {
            let node = self.expression()?;
            self.take(&')');
            Ok(node)
          },
          c if c.is_alphabetic() => {
            Ok(CoxNode::<T> {
              name: self.node_index,
              repr: c.to_string(),
              val: self.eval::<T>(&c).unwrap(),
              mats: Vec::new(),
              cart_vec: None
            })
          }
          _ => Err(SyntaxError::new_parse_error(format!("Not a node at position {}: found {}", "FIXME!", c)))
        }
      }
      None => Err(SyntaxError::new_parse_error("Unexpected end of input".to_string())),
    }
  }

  /// checks for an edge e.g. 3, 5/2
  /// returns a CoxEdge
  fn edge(&mut self) -> Result<CoxEdge, SyntaxError>{
    // Calling edge at the end of a string is a bug
    let p = self.peek().unwrap();

    self.pop();
    if !p.is_numeric() {
      return Err(SyntaxError::new_parse_error(format!("Not a numerical at {}: found {}", "self.cur", p).to_string()));
    }
    let next = self.peek().unwrap();
    if next == '/' {
      self.pop();
      let q = self.pop().unwrap();
      if !q.is_numeric() {
        return Err(SyntaxError::new_parse_error(format!("Not a numerical at {}: found {}", "self.cur", q).to_string()));
      }
      return Ok(CoxEdge::Frac(GenericFraction::<u32>::new(p.to_digit(10).unwrap(),q.to_digit(10).unwrap())));
    }
    Ok(CoxEdge::Int(p.to_digit(10).unwrap()))
  }

  /// Adds a branch to the graph.
  /// Can be one of:
  /// cyclic:
  /// (a) " *<index>" = (b) " *<index><edge>*<index>"
  /// additional branch:
  /// (c) " *<index>[<edge><node>]+"
  /// 
  fn branch<T: Debug + Clone + From<f64>>(&mut self, tope: &mut CoxGraph<T>) -> Result<(), SyntaxError>{
    // take optional space
    self.take(&' ');
    // if there is no '*', there is no branch. exit
    if !self.take(&'*'){
      return Ok(());
    }

    let i = self.index()?;//u32::from(index) - u32::from('a');
    match self.peek() {
      None => {
        // (a): Connect to last edge of tope
        let b = tope.node_count() as u32 - 1;
        tope.add_edge(i.into(), b.into(), CoxEdge::Int(3));
        Ok(())
      },
      Some(_c) => {
        // read the edge value. this increments the cursor
        let e = self.edge()?;
        // Try to take '*' if yes, 
        if self.take(&'*') {
          // (b): " *<i1><edge>*<i2>" Connect i1 to i2
          let b = self.index()?;
          tope.add_edge(i.into(), b.into(), e);
          Ok(())
        } else {
          // (c) "*<i><edge><node>[<edgenode>]+"
          let n = self.node()?;
          let i_n = tope.add_node(n);
          tope.add_edge(i.into(), i_n, e);
          self.edge_nodes(tope, i_n)?;
          Ok(())
        }
      }
    }
  }

  fn index(&mut self) -> Result<u32, SyntaxError> {
    match self.pop() {
      Some(i) if ('a'..='z').contains(&i) =>{
        Ok(u32::from(i) - u32::from('a'))
      },
      _ => {
        Err(SyntaxError::new_parse_error("No index after branch indication".to_string()))
      },
    }
  }

  fn expression<T: Clone>(&mut self) -> Result<CoxNode<T>, SyntaxError>{
    todo!("implement expression");
  }

  ///     x(m,0) = o
  /// x(m,1) = x
  /// x(m,2) = x(m)
  /// esp.     x(2)   = o
  ///          x(3)   = x
  ///          x(4)   = q                q : x  = 1.414214 = sqrt(2)
  ///          x(5)   = f                f : x  = 1.618034 = (1+sqrt(5))/2
  ///          x(5/2) = v                v : x  = 0.618034 = (sqrt(5)-1)/2
  ///          x(6)   = h                h : x  = 1.732051 = sqrt(3)
  ///          x(8)   = k             x(8) : x  = 1.847759 = sqrt[2+sqrt(2)]
  ///          x(8/3)               x(8/3) : x  = 0.765367 = sqrt[2-sqrt(2)]
  ///          x(10)                 x(10) : x  = 1.902113 = sqrt[(5+sqrt(5))/2]
  ///          x(10/3)             x(10/3) : x  = 1.175571 = sqrt[(5-sqrt(5))/2]
  ///          x(12)                 x(12) : x  = 1.931852 = sqrt[2+sqrt(3)]
  ///          x(12/5)             x(12/5) : x  = 0.517638 = sqrt[2-sqrt(3)]
  ///          x(∞)   = u                u : x  = 2
  /// general:                        x(m) : x  = sin(2π/m) / sin(π/m) = 2 cos(π/m) for m>1
  /// x(m,n)
  fn eval<T: Clone + From<f64>>(&mut self, c: &char) -> Result<T, SyntaxError> {
    // TODO: make this a HashMap
    match c {
      'o' => Ok(T::from(0f64)),
      'x' => Ok(T::from(1f64)),
      'q' => Ok(T::from(2f64.sqrt())),
      'f' => Ok(T::from((1f64+5f64.sqrt())/2f64)),
      'v' => Ok(T::from((5f64.sqrt()-1f64)/2f64)),
      'h' => Ok(T::from(3f64.sqrt())),
      'k' => Ok(T::from((2f64+2f64.sqrt()).sqrt())),
      'u' => Ok(T::from(2f64)),
      'F' => Ok(T::from((3f64+5f64.sqrt())/2f64)),
      _ => Err(SyntaxError::new_parse_error(format!("Unrecognized symbol: {}", c).to_string()))
    }
  }

  pub fn cursor(&self) -> usize {
    self.cur
  }

  pub fn peek(&self) -> Option<char> {
    match self.chars.get(self.cur) {
      Some(c) => {
        Some(*c)
      },
      None => None,
    }
  }

  pub fn is_done(&self) -> bool {
    self.cur == self.chars.len()
  }

  pub fn pop(&mut self) -> Option<char> {
    match self.chars.get(self.cur) {
      Some(c) => {
        self.cur += 1;

        Some(*c)
      },
      None => None,
    }
  }

  /// Returns true if the `target` is found at the current cursor position,
  /// and advances the cursor.
  /// Otherwise, returns false leaving the cursor unchanged.
  pub fn take(&mut self, target: &char) -> bool {
    match self.chars.get(self.cur) {
      Some(character) => {
        if target == character {
          self.cur += 1;

          true
        } else {
          false
        }
      }
      None => false,
    }
  }

  /// Invoke `cb` once. If the result is not `None`, return it and advance
  /// the cursor. Otherwise, return None and leave the cursor unchanged.
  pub fn transform<T>(
    &mut self,
    cb: impl FnOnce(&char) -> Option<T>,
  ) -> Option<T> {
    match self.chars.get(self.cur) {
      Some(input) => match cb(input) {
        Some(output) => {
          self.cur += 1;

          Some(output)
        },
        None => None
      },
      None => None
    }
  }
}

