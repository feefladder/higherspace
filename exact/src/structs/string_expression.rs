// use crate::expr_field::{FlatField, self, FlatRef};

/// an expression that can be parsed
pub struct StrExpr {
  ch: char,
  asc_ch: char,
  ascii: String,
  expr: StringOrField,
}

/// The expression is a topologically sorted, self-contained field
/// e.g. 2π²-> [π,2π,2π²]
///        \-> [2π²,2π,π]
pub enum StringOrField {
  Str(String),
  // Expr(expr_field::Expr<'static, FlatRef<'static>, FlatField<'static>>),
}