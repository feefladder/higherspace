use crate::expr_rc::{
  SVec,
  simplify::collect_like,
};

pub fn collect_like_terms(v: SVec) -> SVec{
  collect_like(v, |(_,f1),(_,f2)| *f1 == f2, |(c1,_),(c2,_)| *c1 += c2)
}
