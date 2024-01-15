/// Name of an Expr variant. Should return char
/// Val  -> ξ
/// Const -> <char of const>
/// Sum  -> Σ
/// Prod -> Π
/// Sqrt -> √
/// For sin, cos, tan, they are a clock based on this triangle:
/// ```raw
/// Cos   Sin   Tan
/// ____
/// |  /  /|    /|
/// |-/  / |   / |
/// |/  /\_|  /\_|
/// Cos Sin Tan
/// |/  /|  _|
/// 🕑  🕢  🕘
/// Sin  -> 🕢
/// Cos  -> 🕑
/// Tan  -> 🕘
/// ```
pub trait Char {
  fn ch(&self) -> char;
}

pub trait AsciiString {
  fn ascii(&self) -> &str;
}