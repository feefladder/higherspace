// #[macro_use]
// extern crate lazy_static;

use petgraph::stable_graph::EdgeIndex;
// use petgraph::adj::EdgeIndex;
// use rusqlite::{Connection, Result};
use regex::RegexSet;
use fraction::{GenericFraction, ToPrimitive, Fraction, Num};
use ndarray_linalg::Determinant;

use std::fmt::{Debug, Formatter, Display};
use std::process::{Command, Stdio, ExitStatus};
use std::io::Write; // Import Write trait for write_all
use std::convert::From;
use std::f64::consts::PI;
use std::str::FromStr;

use petgraph::graph::{UnGraph, NodeIndex, Node};
use petgraph::algo::{is_isomorphic_subgraph_matching, subgraph_isomorphisms_iter};

use exact::F;

// use crate::parse::parse;
use crate::parse::Scanner;

pub mod parse;

pub type CoxGraph<T> = UnGraph<CoxNode<T>, CoxEdge, u32>;

/// The set of irreducible discrete reflection-generated pointgroups of arbitrary dimension are known to be the following Coxeter groups. The left group names are according to Coxeter himself; in parantheses the slightly differing Lie group names are added.
///
/// A(n)  (= An)    or  o3o...o3o                                                        (n>0 nodes)
/// 
/// B(n)  (= Dn)    or  o3o...o3o *b3o      =  o3o...o3o *-c3o      =  o3o3o *b3o...o3o  (n>3 nodes)
/// 
/// C(n)  (= Bn)    or  o3o...o3o4o         =  o4o3o...o3o                               (n>1 nodes)
/// 
/// D(2)p (= I2(p)) or  oPo
/// 
/// E(6)  (= E6)    or  o3o3o3o3o *c3o      =  o3o3o3o3o *-d3o      =  o3o3o3o *b3o3o
/// 
/// E(7)  (= E7)    or  o3o3o3o3o3o *c3o    =  o3o3o3o3o3o *-d3o    =  o3o3o3o *b3o3o3o
/// 
/// E(8)  (= E8)    or  o3o3o3o3o3o3o *c3o  =  o3o3o3o3o3o3o *-d3o  =  o3o3o3o *b3o3o3o3o
/// 
/// F(4)  (= F4)    or  o3o4o3o
/// 
/// G(3)  (= H3)    or  o3o5o
/// 
/// G(4)  (= H4)    or  o3o3o5o
#[derive(Debug, Clone, Default)]
pub struct CoxGroup<'a, T: Clone>{
  graph: CoxGraph<T>,
  polys: Vec<Polygon<'a, T>>,
}

#[derive(Debug, Clone)]
pub struct Polygon<'a, T>{
  m1: &'a CoxNode,
  m2: &'a CoxNode,
  verts: Vec<&'a ndarray::Array2<T>>,
  m1_edges: Vec<(usize,usize)>,
  m2_edges: Vec<(usize,usize)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CoxEdge{
  Int(u32),
  Frac(GenericFraction<u32>),
}

impl From<&CoxEdge> for f64 {
  fn from(edge: &CoxEdge) -> f64 {
    match edge {
        CoxEdge::Frac(f) => {
          return f.to_f64().unwrap();
        },
        CoxEdge::Int(i) => {
          return *i as f64;
        }
    }
  }
}

impl Display for CoxEdge {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      CoxEdge::Int(int) => {
        f.write_fmt(format_args!("{}", *int))?;
        Ok(())
      },
      CoxEdge::Frac(frac) => {
        f.write_fmt(format_args!("{}/{}", frac.numer().unwrap(), frac.denom().unwrap()))?;
        Ok(())
      },
    }
  }
}

impl<T: Clone> Display for CoxNode<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}", self.repr))?;
    Ok(())
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CoxNode<T: Clone =f64>{
  name: char,
  repr: String,
  val: T,
  mats: Vec<ndarray::Array2<T>>,
  cart_vec: Option<ndarray::Array1<T>>,
}

// struct Frac<T>{
//     sign: i8,
//     p: T,
//     q: T,
// }

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
// impl<T: std::ops::Mul<Output = T> + Copy> ops::Mul<&Frac<T>> for &Frac<T> {
//     type Output = Frac<T>;
//     fn mul(self, _rhs: &Frac<T>) -> Frac<T>{
//         return Frac::<T> {
//             sign: self.sign * _rhs.sign,
//             p: self.p * _rhs.p,
//             q: self.q * _rhs.q,
//         }
//     }
// }

// fn lcm<T: std::ops::Mul<Output = T> + Copy>(a: &T, b: &T) -> T {
//     // TODO: actually do lcm
//     return (*a)*(*b);
// }

// impl<T: 
//     std::ops::Add<Output = T> +
//     std::ops::Sub<Output = T> +
//     std::ops::Mul<Output=T> +
//     std::ops::Div<Output=T> + 
//     std::cmp::PartialOrd +
//     Copy
// > ops::Add<&Frac<T>> for &Frac<T> {
//     type Output = Frac<T>;
//     fn add(self, _rhs: &Frac<T>) -> Frac<T>{
//         let q = lcm(&self.q, &_rhs.q);
//         let ps = self.p * (q/self.q);
//         let pr = _rhs.p * (q/_rhs.q);
//         let p: T; let sign: i8;
//         if self.sign == _rhs.sign {
//             p = ps + pr;
//             sign = self.sign;
//         } else if ps > pr {
//             p = ps - pr;
//             if self.sign == 1 {
//                 sign = 1;
//             } else {
//                 sign = -1;
//             }
//         } else {
//             p = pr - ps;
//             if self.sign == 1 {
//                 sign = -1;
//             } else {
//                 sign = 1;
//             }
//         }
//         // let new_p = self.sign == _rhs.sign ? ps + pr : ps > pr ? ps - pr : pr - ps;
//         // let new_sign = 
//         return Frac::<T> {
//             sign: sign,
//             p: p,
//             q: q,
//         }
//     }
// }

// The set of irreducible discrete reflection-generated pointgroups of arbitrary dimension are known to be the following Coxeter groups. The left group names are according to Coxeter himself; in parantheses the slightly differing Lie group names are added.

// A(n)  (= An)    or  o3o...o3o                                                        (n>0 nodes)
// B(n)  (= Dn)    or  o3o...o3o *b3o      =  o3o...o3o *-c3o      =  o3o3o *b3o...o3o  (n>3 nodes)
// C(n)  (= Bn)    or  o3o...o3o4o         =  o4o3o...o3o                               (n>1 nodes)
// D(2)p (= I2(p)) or  oPo
// E(6)  (= E6)    or  o3o3o3o3o *c3o      =  o3o3o3o3o *-d3o      =  o3o3o3o *b3o3o
// E(7)  (= E7)    or  o3o3o3o3o3o *c3o    =  o3o3o3o3o3o *-d3o    =  o3o3o3o *b3o3o3o
// E(8)  (= E8)    or  o3o3o3o3o3o3o *c3o  =  o3o3o3o3o3o3o *-d3o  =  o3o3o3o *b3o3o3o3o
// F(4)  (= F4)    or  o3o4o3o
// G(3)  (= H3)    or  o3o5o
// G(4)  (= H4)    or  o3o3o5o
fn check_input(haystack: &String) {
  let patterns = [
    // An
    r"^(?:[\p{Letter}]+3)+[\p{Letter}]$",
    // Dn
    r"^(?:[\p{Letter}]+3)+[\p{Letter}]+[ ]?\*(?:b||-c)3[\p{Letter}]+$",
    r"^[\p{Letter}]+3[\p{Letter}]+3[\p{Letter}]+[ ]?\*b(?:3[\p{Letter}]+)+$",
    // Bn
    r"^(?:[\p{Letter}]+3)+[\p{Letter}]+4[\p{Letter}]+$",
    r"^[\p{Letter}]+4[\p{Letter}]+(?:3[\p{Letter}]+)+$",
    // I2(p)
    r"^[\p{Letter}]+[0-9]+[\p{Letter}]+$",
    // E6
    r"^[\p{Letter}]+(?:3[\p{Letter}]+){4}[ ]?\*(?:b||-c)3[\p{Letter}]+$",
    // E7
    r"^[\p{Letter}]+(?:3[\p{Letter}]+){5}[ ]?\*(?:b||-c)3[\p{Letter}]+$",
    // E8
    r"^[\p{Letter}]+(?:3[\p{Letter}]+){6}[ ]?\*(?:b||-c)3[\p{Letter}]+$",
    // F4
    r"^[\p{Letter}]+3[\p{Letter}]+4[\p{Letter}]+3[\p{Letter}]+$",
    // H3
    r"^[\p{Letter}]+3[\p{Letter}]+5[\p{Letter}]+$",
    r"^[\p{Letter}]+5[\p{Letter}]+3[\p{Letter}]+$",
    // H4
    r"^[\p{Letter}]+3[\p{Letter}]+3[\p{Letter}]+5[\p{Letter}]+$",
    r"^[\p{Letter}]+5[\p{Letter}]+3[\p{Letter}]+3[\p{Letter}]+$",
  ];
  let re_set = RegexSet::new(patterns).unwrap();
  let matches: Vec<_> = re_set.matches(haystack).into_iter().collect();
  println!("{:?}",matches);
  // let re_an = Regex::new();
  // let re_dn = Regex::new();
  // let re_bn = Regex::new();
  // // oPo
  // let re_e6 = Regex::new();
  // let re_e7 = Regex::new();
  // let re_e8 = Regex::new();
  // let re_f4 = Regex::new();
  // let re_h3 = Regex::new();
  // let re_h4 = Regex::new();
}

/// Initialize the first matrices for each node
fn init_matrices<T: Clone + fraction::Zero + fraction::One>(symm: &mut CoxGraph<f64>) -> Result<(), ()>{
  // let mut nodes = symm.raw_nodes();
  let dims = symm.node_count();
  for i in 0..dims {
    let mut mat: ndarray::Array2<f64> = ndarray::Array2::<f64>::eye(dims);
    for j in 0..dims {
      if i == j {
        mat[[j,i]] = -1.0;
      } else {
        match symm.find_edge((i as u32).into(), (j as u32).into()) {
          Some(e) => {
            match symm.edge_weight(e).unwrap() {
              CoxEdge::Int(c) => {
                mat[[j,i]] = 2.0*(PI/f64::from(*c)).cos();
              },
              CoxEdge::Frac(f) => {
                mat[[j,i]] = 2.0*(PI*f64::from(*f.denom().unwrap())/f64::from(*f.numer().unwrap())).cos();
              }
            }
          },
          None => {
            // implicit 2 edge, should be zero: cos(pi/2)=0
            assert_eq!(mat[[i,j]], 0.0);
          }
        }
      }
    }
    println!("{}", mat);
    symm[NodeIndex::new(i)].mats.push(mat);
  }
  Ok(())
}

/// Reflect matrices for each node
/// Maybe also perform calculations
/// Also check if matrix already exists somewhere else in the graph
/// TODO: improve data structure
/// Given the diagram
/// a1b2c
/// node a indicates a distance from mirror A, which has angle 𝛑/1 with mirror B
/// reflecting A in B is done by
/// BAB
/// to obtain A010
/// reflecting A in C is done by
/// CAC
/// to obtain
/// A001
/// To obtain A011, we have to reflect in B and then C or not vice versa
/// CBABC =/= BCACB
/// Let's see. If we have a symmetry given by aibjc*ak*c, then we have up to now
/// a: CoxNode with: val:f64 mats<f64>[[-1,0,0],[2ci,1,0],[2cj,0,1]], cart_vec [xa,ya,za(,wa=0)]
/// b: CoxNode with: val:f64 mats<f64>[[1,2ci,0],[0,-1,0],[0,2cj,1]], cart_vec [xb,yb,zb]
/// c: CoxNode with: val:f64 mats<f64>[[1,0,2ck],[0,1,2cj],[0,0,-1]], cart_vec [xc,yc,zc]
/// Now, nodes also correspond to a mirror, a->A,b->B,c->C and the effect of mirror A in mirror-space is given by the matrix of node a.
/// point p is given in mirror space by: [a.val,b.val,c.val]T
/// the cartesian coordinates of any p are given by [a.cart_vec,b.cart_vec,c.cart_vec].dot([a.val,b.val,c.val])
/// What needs to be done is:
/// A1=BAB  B
///   \  . /
///  . \  / .p
/// ____\/____A
///B1   /\
///  . /  \ .
///   / .  \
///  A2     B2
/// 1. reflect point P in mirror B iff b.val != 0 to yield edge Eb0
///    reflect edge Eb0 in mirror A1=BAB iff a.val !=0 to yield Eb1. and Ea1
///    reflect edge Eb1 in mirror A2 to yield Eb2 and Ea2
///    ...repeat until P(ababab)+=P
/// 2. this gives (non-lacing) edges for polygon ab
/// 3. return coordinates in cartesian space
#[allow(non_snake_case)]
fn reflect_matrices<T>(symm: &mut CoxGraph<f64>) -> Result<(),()> {
  let A = &symm[NodeIndex::new(0)].mats[0];
  let B = &symm[NodeIndex::new(1)].mats[0];
  let C = &symm[NodeIndex::new(2)].mats[0];

  let n: &u32;
  match symm.edge_weight(symm.find_edge(NodeIndex::new(0), NodeIndex::new(1)).unwrap_or(EdgeIndex::new(usize::MAX))) {
    Some(a) => {
      match a {
        CoxEdge::Int(a) => {
          n = a;
        },
        CoxEdge::Frac(a) => {
          n = a.numer().unwrap();
        }
      }
    },
    None => {
      n = &2;
    }
  }

  let mut mats: Vec<Box<ndarray::Array2<f64>>> = Vec::new();
  // Create polygon AB:
  // reflect p in B
  mats.push(Box::new(A.clone()));
  mats.push(Box::new(B.clone()));
  for i in 1..*n {
    if i % 2 == 1 {
      // reflect A in B
      let Bi = &*mats[mats.len()-1];
      let Ai = &*mats[mats.len()-2];
      let Ai1= Box::new(Bi.dot(&Ai.dot(&*Bi)));
      mats.push(Ai1);
    } else {
      // reflect B in A
      let Ai = &*mats[mats.len()-1];
      let Bi = &*mats[mats.len()-2];
      let Bi1= Box::new(Ai.dot(&Bi.dot(Ai)));
      mats.push(Bi1);
    }

    // println!("p: {:?}, i: {:?}, mats: {:?}", n,i, mats);
  }
  if mats[0].abs_diff_eq( &mats[mats.len()-1], 1e-15) {
    println!("Yay! we're in some poly thingy");

  } else {
    println!("Nay! we're not in some poly thingy {:?}", (*mats[0]).clone()-(*mats[mats.len()-1]).clone());
  }
  Ok(())
}

/// Determines the basis vectors for a symmetry. Derived from exact_cartesian.pdf
/// First checks isomorphism with part of the graph that corresponds to either cubic, icosahedral or tetrahedal symmetry
/// Then determines basis vectors for each of the nodes. E.g.
/// x4o3o -> (1/2,1/2,1/2) (cube)
/// o4x3o -> (1/√2,0,1/√2) (cuboctahedron)
/// o4o3x -> (0,0,1/√2)    (octahedron)
/// φ = (1+√5)/2
/// x5o3o -> (1/2,0,φ²/2)       (dodecahedron)
/// o5x3o -> (0,0,φ)  (icosidodecahedron)
/// o5o3x -> (0, 1/2, φ/2) (icosahedron)
/// 
/// x3o3o -> (0,1/2,√2/4) (tet)
/// o3x3o -> (0,0,√2/2)   (oct)
/// o3o3x -> (1/2,0,√2/4) (tet)
/// One could ask, why not directly closed-form exact expressions?
/// Because there is the secret hope of being able to derive cartesian coordinates for higher-dimensional polytopes as well
/// But well... it's taken some time up to now...
fn determine_cartesian_3d(symm:& mut CoxGraph<f64>) -> Result<(), ()> {
  // let dims=symm.node_count();
  // assert_eq!(symm.node_count(),3);
  // derivations are in exact_cartesian.pdf
  /* reference is x4o3o -> a4b3c -> (.5,.5,.5)
    A B C
    o4x3o -> (1/sqrt(2),0,1/sqrt(2))
    o4o3x -> (0,0,1/sqrt(2))
     | |
     r q p=2
    x2o5o*a3*c
    Q P R
    x2o3o*a3*c

     p q r=2
    o3o5o
    o3o3o

    o3o4o
     q r p=2
   */
  //  p q r=2
  // a1b2c
  // Q R P
  //  r q p=2
  // o3o4o
  // Q P R
  // [1,2,0]
  //  p q   r
  // o2o3o*a4*c
  // Q R P
  let cubic:&CoxGraph<f64> = &UnGraph::from_edges(&[(1,2,CoxEdge::Int(3)),(0,2,CoxEdge::Int(4))]);
  //  p q r=2
  // o3o3o
  // Q R P
  // [2,0,1]
  let tetrahedal:& CoxGraph<f64> = &UnGraph::from_edges(&[(0,1,CoxEdge::Int(3)),(1,2,CoxEdge::Int(3))]);
  //  p q r=2
  // o3o5o
  // Q R P
  // [2,0,1]
  let icosahedral:& CoxGraph<f64> = &UnGraph::from_edges(&[(0,1,CoxEdge::Int(3)),(1,2,CoxEdge::Int(5))]);

  let mut n_eq = |_: &_,_: &_| true;
  let mut e_eq = |a: &_,b: &_| a==b;

  // non-mutable reference
  let g1 = &*symm;
  
  // (sub)symmetry
  let the_symm: &CoxGraph::<f64>;
  // isomorphism mapping
  let is: Vec<usize>;
  // edge values
  // let p: f64; let q: f64; let r: f64;
  if is_isomorphic_subgraph_matching(&cubic, &*symm, n_eq, e_eq) {
    println!("Cubic subsymmetry!");
    // p=2, q=3, r=4
    // Find the isomorphism
    let mut it = subgraph_isomorphisms_iter(&cubic, &g1, &mut n_eq, &mut e_eq).unwrap();
    is = it.next().unwrap();
    the_symm = cubic;
  } else if is_isomorphic_subgraph_matching(&tetrahedal, &*symm, |_,_| true, |a,b| a==b) {
    
    // p=q=3, r=2
    let mut it = subgraph_isomorphisms_iter(&tetrahedal, &g1, &mut n_eq, &mut e_eq).unwrap();
    is = it.next().unwrap();
    the_symm = tetrahedal;
    println!("Tet subsymmetry {:?}", is);
  } else if is_isomorphic_subgraph_matching(&icosahedral, &*symm, |_,_| true, |a,b| a==b) {
    // p=3, q=5, r=2
    println!("ico subsymmetry");
    let mut it = subgraph_isomorphisms_iter(&icosahedral, &g1, &mut n_eq, &mut e_eq).unwrap();
    is = it.next().unwrap();
    the_symm = icosahedral;
  } else {
    println!("Unknown subsymmetry");
    is = vec![0,1,2];
    the_symm = symm;
  }

  // first, construct the H² matrix
  let p = f64::from(the_symm.edge_weight(the_symm.find_edge(0.into(), 1.into()).unwrap_or(u32::MAX.into())).unwrap_or(&CoxEdge::Int(2)));
  let q = f64::from(the_symm.edge_weight(the_symm.find_edge(1.into(), 2.into()).unwrap_or(u32::MAX.into())).unwrap_or(&CoxEdge::Int(2)));
  let r = f64::from(the_symm.edge_weight(the_symm.find_edge(0.into(), 2.into()).unwrap_or(u32::MAX.into())).unwrap_or(&CoxEdge::Int(2)));
  // pre-determine cp := cos(𝛑/p), sp := sin(𝛑/p) etc.
  let cp = (PI/p).cos(); let cq = (PI/q).cos(); let cr = (PI/r).cos();
  let sp = (PI/p).sin(); let sq = (PI/q).sin(); let sr = (PI/r).sin();

  #[allow(non_snake_case)]
  let H = ndarray::Array2::<f64>::from(vec!
  [
    [1.0,-cq,-cr],
    [-cq, 1.0, -cp],
    [-cr, -cp, 1.0]
  ]).det().unwrap_or(0.0).sqrt();
  if true {//H.is_nan() {
    println!("{:?},{:?},{:?},{:?},{:?}",cq,cr,cp,(1f64-cp*cp)+cq*(-cq-cp*cr)-cr*(cp*cq+cr), H);
    // return Err(());
  }
  println!("p: {:?}, q: {:?}, r: {:?}, H: {:?}", p,q,r,H);
  
  #[allow(non_snake_case)]
  let P = ndarray::Array1::from(vec![
    H/(sr*sp),
    0.0,
    (cq+cr*cp)/(sr*sp),
  ]);
  #[allow(non_snake_case)]
  let Q = ndarray::Array1::from(vec![
    H/(PI/r).tan(),
    H,
    (cp+cq*cr)/sr,
  ]) / sq;
  #[allow(non_snake_case)]
  let R = ndarray::Array1::from(vec![
    0f64,
    0f64,
    1f64,
  ]);

  // Normal vectors to planes
  let n_qr = ndarray::Array1::from(vec![sr,-cr,0f64]);
  let n_pr = ndarray::Array1::from(vec![0f64,1f64,0f64]);
  let n_pq = ndarray::Array1::from(vec![(-cq-cp*cr)/sr,-cp,H/sr]);
  
  // So we are working on the_symm, now it is time to map the obtained cartesian coords into the provided symmetry
  // 
  symm[NodeIndex::new(is[0])].cart_vec = Some(Q.clone()/Q.dot(&n_pr)/2f64);
  symm[NodeIndex::new(is[1])].cart_vec = Some(R.clone()/R.dot(&n_pq)/2f64);
  symm[NodeIndex::new(is[2])].cart_vec = Some(P.clone()/P.dot(&n_qr)/2f64);
  
  println!("{:?}?{:?}?{:?}",
    symm[NodeIndex::new(0)].cart_vec,
    symm[NodeIndex::new(1)].cart_vec,
    symm[NodeIndex::new(2)].cart_vec);
  Ok(())
}


fn print_svg<N: std::fmt::Display,E: std::fmt::Display>(graph: &UnGraph<N, E>) -> ExitStatus{
  let status = Command::new("dot")
    .arg("-Tsvg")  // Specify the output format (SVG)
    .arg("-o")
    .arg("graph.svg")
    .stdin(Stdio::piped())
    .spawn()
    .and_then(|mut child| {
      child.stdin.take().unwrap().write_all(format!("{}", petgraph::dot::Dot::new(&graph)).as_bytes())?;
      Ok(child.wait()?) // Wait for the child process to finish
    })
    .expect("Failed to execute dot command");
  status
}

fn main() -> Result<(),()>{
  // parse::<f64>("hello!");
  // let mut sc = Scanner::new("x2o5o*a3*c");
  // let mut sc = Scanner::new("x4o3o");
  let mut sc = Scanner::new("x5f3o");
  let mut group: CoxGroup<f64> = sc.parse_tope::<f64>().unwrap();
  println!("{:?}", petgraph::dot::Dot::new(&group.graph));
  println!("{:?}", ndarray::Array2::from(Vec::from(&[[1,2,3],[4,5,6]])));
  // group.build_symmetry();
  // let conn = Connection::open("cats.db")?;
  init_matrices::<f64>(&mut group.graph)?;

  println!("{:?}", petgraph::dot::Dot::new(&group.graph));
  print_svg(&group.graph);
  determine_cartesian_3d(&mut group.graph)?;

  reflect_matrices::<f64>(&mut group.graph)?;
  println!("{:?}", f64::try_from(GenericFraction::<u32>::new(1u8,2u8)));
  
  // use std::rc::Rc;

  println!("¹³/₂");

  // let data = 5;
  // let mut rc_a = Rc::new(data);
  // let rc_b = Rc::new(data);
  // let c =  Rc::<i32>::get_mut(&mut rc_a).unwrap();
  // *c += 5;
  // println!("a: {:p}:{:?}, {:p},{:p}", rc_b, rc_b, rc_a, &data);
  // let f = F::new(1u8,2u8);
  // println!("pretty: {}, debug: {:?}, pretty debug: {:#?}",-f,f,f);
  // println!("{}", (PI*5.0).sin());

  // let t_str: Vec<&str> = vec![
  //   "10",
  //   "10 and some text",
  //   "0xdeadbeef",
  //   ""
  // ];
  // for t in t_str {
  //   println!("parsed string: '{}' to {:?}", t, t.to_string().parse::<u32>());
  // }

  // let t_str: Vec<&str> = vec![
  //   "1/2",
  //   "10 and some text",
  //   "deadbeef",
  //   ""
  // ];
  // for t in t_str {
  //   println!("parsed string: '{}' to {:?}", t, t.parse::<Fraction>() );
  // }
  // use 
  // iso_map(&mut group.graph)?;
  // conn.execute(
  //     "create table if not exists cat_colors (
  //          id integer primary key,
  //          name text not null unique
  //      )",
  //     [],
  // )?;
  // conn.execute(
  //     "create table if not exists cats (
  //          id integer primary key,
  //          name text not null,
  //          color_id integer not null references cat_colors(id)
  //      )",
  //     [],
  // )?;

  Ok(())
}


// struct Foo;
// struct Bar;

// #[derive(Debug)]
// struct FooBar;

// #[derive(Debug)]
// struct BarFoo;


// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         println!("> Foo.add(Bar) was called");

//         FooBar
//     }
// }

// // By reversing the types, we end up implementing non-commutative addition.
// // Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// // This block implements the operation: Bar + Foo = BarFoo
// impl ops::Add<Foo> for Bar {
//     type Output = BarFoo;

//     fn add(self, _rhs: Foo) -> BarFoo {
//         println!("> Bar.add(Foo) was called");

//         BarFoo
//     }
// }

// fn main() {
//     println!("Foo + Bar = {:?}", Foo + Bar);
//     println!("Bar + Foo = {:?}", Bar + Foo);
// }
