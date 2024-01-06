use std::collections::LinkedList;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::{
  Index,
};

type Chunk<DType, const CHSize: usize> = [MaybeUninit<DType>; CHSize];
type ArrORGType<DType, const CHSize: usize> = LinkedList<Chunk<DType, {CHSize}>>;



/// Array of 'immutable' elements that can only grow
/// allows for shared ownership and contiguousness
/// Does not get copied anywhere
/// ### Example if CHUNK_SIZE == 3
/// ```raw
/// /-CHUNK-\             /-uninitialized
/// [π,π²,½π²]<-->[e,eπ²,..]-linked list of arrays
///   \-ref is maybe a raw pointer to this address
/// ```
struct ArrayOnlyReadGrow<'a, DType, const CHSize: usize> {
  /// The index of the last element in the last chunk
  /// ```raw
  ///  0  1  2       0  1
  /// [π,π²,½π²]<-->[e,eπ²,...]
  ///                   \-1
  /// ```
  arr_i: usize,
  /// The index of the last chunk in the list
  /// ```raw
  /// /---0---\     /----1----\
  /// [π,π²,½π²]<-->[e,eπ²,...]
  ///             1-/---------/
  /// ```
  chunk_i: usize,
  /// The "array": linked list of arrays
  /// ```raw
  /// [π,π²,½π²]<-->[e,eπ²,...] <- this thing
  /// ```
  arr: ArrORGType<DType, {CHSize}>,
}

impl<'a, DType, const CHSize: usize> ArrayOnlyReadGrow<'a, DType, { CHSize }> {
  // type CHType = Chunk<DType, {CHSize}>;

  /// Create new array
  pub fn new() -> Self {
    let arr_org = ArrayOnlyReadGrow::<DType>{
      arr_i: 0,
      chunk_i: 0,
      arr: ArrORGType::new(),
    };
    arr_org.push_chunk();
    arr_org
  }

  /// Push an uninitialized chunk to the list
  fn push_chunk(&self) {
    self.chunk_i += 1;
    self.arr_i = 1;
    self.arr.push_back(self.alloc_chunk())
  }

  /// Create an uninitialized array of `MaybeUninit`. The `assume_init` is
  /// safe because the type we are claiming to have initialized here is a
  /// bunch of `MaybeUninit`s, which do not require initialization.
  /// see [docs](https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element)
  fn alloc_chunk() -> &'a mut Chunk<DType, {CHSize}> {
    let mut data: Chunk<DType, {CHSize}> = unsafe { MaybeUninit::uninit().assume_init() };
    &mut data
  }

  /// Add a value to the array
  /// ```raw
  /// /---0---\     /----1----\
  /// [π,π²,½π²]<-->[e,eπ²,½eπ²]
  ///            1--/----------/
  /// ```
  pub fn add(&self, val: DType){
    if self.arr_i == CHSize {
      self.push_chunk();
    }
    self.arr[self.chunk_i][self.arr_i] = val;
  }

  // pub fn append(&self, vals: )
}

impl<'a, DType: 'a + Sized, Idx, const CHSize: usize> Index<Idx> for ArrayOnlyReadGrow<'a, DType, { CHSize }> {
  type Output = &'a DType;

  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    if index > self.chunk_i*CHSize + self.arr_i {
      panic!("Tried to index {} at {:?}", self, index)
    } else {
      let ch_i = index / CHSize;
      let arr_i = index % CHSize;
      let ch = self.arr.back().unwrap();
      unsafe { ch.get_unchecked(index) }
    }
  }
}

impl<'a, DType, const CHSize: usize> Debug for ArrayOnlyReadGrow<'a, DType, {CHSize}> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "TODO")
  }
}
