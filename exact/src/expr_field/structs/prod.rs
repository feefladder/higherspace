use crate::expr_field::{structs::Prod, FieldTrait};

impl<'a, Field: FieldTrait<'a>> Eq for Prod<'a, Field> {}
impl<'a, Field: FieldTrait<'a>> PartialEq for Prod<'a, Field> {
  #[inline]
  fn eq(&self, other: &Prod<'a, Field>) -> bool {
    self.factors == other.factors
  }
}

impl <'a, Field: FieldTrait<'a>> PartialOrd for Prod<'a, Field> {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl <'a, Field: FieldTrait<'a>> Ord for Prod<'a, Field> {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.factors.cmp(&other.factors)
  }
}