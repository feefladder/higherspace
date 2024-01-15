# Backlog (in no particular order)

## Sum series notation

```
Σ_n=0^2[Π(a,n)] <-> Σ[(1,ξ1),Π(ξa,1),Π(ξa,2)]
->: .expand(); <-: .try_
name 'n' and "=0" are implicit:
  Σ_n=0^2[Π(a,n)] = Σ_0^2[Π(a,n)] = Σ^2[Π(a,n)]
Caret is optional if only upper limit:
  ... = Σ2[Π(a,n)]
Implicitly no upper limit:
  Σ[Π(a,n)].expand() =GSeries= Π(something)
```

## parsing

### The problem: 1. delegate parsing

I use `fraction` crate. Fraction crate and Rust number system knows how to parse fractions and numbers. Also, in `polytope` crate, expressions can be used in between "mirrors":
```
/---polytope parser-----\
|/---exprparse------\   |
||   /-fparse-\     |   |
||  / \      / \    |   |
 Σ[(1/2,ξ1),(1/2,√5)]5φ3o
```
`exprparse` can finish when it signals the closing expression, but fraction and int don't do that:
```
parsed string: '10' to Ok(10)
parsed string: '10 and some text' to Err(ParseIntError { kind: InvalidDigit })
parsed string: '0xdeadbeef' to Err(ParseIntError { kind: InvalidDigit })
parsed string: '' to Err(ParseIntError { kind: Empty })
```
types:

1. *Reluctant*: will stop whenever it has a successful function.
2. *Eager*: Will continue parsing until everything is broken. <- this is the `String.parse<{integer}>("9001 Oh No!")` type.

Take a look at Fraction crate
### The solution: "delimiters"
```
/---polytope parser-----\
|/---exprparse------\   |
||   /-fparse-\     |   |
||  / \      / \    |   |
 Σ[(1/2,ξ1),(1/2,√5)]5φ3o
   /   \-error if there is no comma, or anything between 
 '('and',' cannot be parsed by str.parse::<F>("1/2")
```

```
/---polytope parser-----\
|/---exprparse------\   |
||   /-fparse-\     |   |
||  / \      / \    |   |
 Σ[(1/2,ξ1),(1/2,√5)]3φ3o
'3' is not a delimiter\/
It could be used inside expr
needs exprparse to be "Reluctant"
```
Could also use a delimiter queue to check for matching bracket types.

add some function to the parser, probably a wrapper around "scan"
```
while sc.peek().unwrap() != &']' {
  //do stuff
}
```
Like this:
```
/// ch_term: terminating char
fn super_scan_ch(ch_term: &char) -> &str {
  let start = self.cursor();
  while sc.peek().unwrap() != &ch { // \_-Add end-of-line error handling
    sc.pop();                       // /
  }
  self.string[start..self.cursor()]
}

/// not sure if negation is the right way
/// What's a good name for stop_if: take_if?
fn super_scan(stop_if: impl FnMut(char) -> bool) -> &str {
  let start = self.cursor();
  while !stop_if(sc.peek().unwrap()) { // \_-Add end-of-line error handling
    sc.pop();                          // /
  }
  self.string[start..self.cursor()]
}

fn super_scan_ch(stop_ch: &char) -> &str {
  super_scan(|c| c == stop_ch)
}

fn super_scan(go_on: impl FnMut(char) -> bool, err_if: fn->bool) -> &str {
  let start = self.cursor();
  while go_on(sc.peek().unwrap()) { // \_-Add end-of-line error handling
    err_if(sc.pop())?;                          // /
  }
  self.string[start..self.cursor()]
}
```
It is still single look
or something that takes a `|&char| -> bool` for further processing.
```rust
  // "Hello Scanner!"
  // ^
  let substr = scanner.super_scan(|c| *c != ' ')?;
  //      \- "Hello"
  //               ^-cursor points at ' '
  let &mut word_scanner = OtherScannerType::new();
  Ok()
```
### ASCII
Add an option for pure-ascii.
```

/// Human-readable machine-to-machine character
/// Preferably in some range
/// would be nice to make a very simple copy-paste applet
/// but actually a text file would be good enough
/// so a user can copy-paste special chars like sqrt
pub trait Char {
  fn ch() -> &str;
}


/// human-read-writeable representation
/// The same as LaTeX if it exists
pub trait ASCII {
  // some fancy stuff to ensure ascii-nes, or not
  fn ascii() -> &str;
}


/// machine-to-machine communication
pub trait AsciiChar {
  fn asc_ch() -> &str;
}
|ch |ascii  |asc_ch |description |
|'O'|"O"    |'O'    |Zero        |--\
|'I'|"I"    |'i'    |One         |  |-- adding these will make 
|'' |"Nan"  |'N'    |Not a Number|  |   logic nicer
|'' |"Infty"|'I'    |Infinity (⨥)|--/
|'ξ'|"Val"  |'V'    |Value       |
|π,e|pi,e   |'p','e'|Constant    |
|'Σ'|"Sum"  |'S'    |Sum         |
|'Π'|"Prod" |'P'    |Prod        |
|   |Sqrt,Cs|'q','C'|Function    |
```
Probably a proc macro around the exsting scan function:
```rust
fn_scan![
  "Sum" => {Ok(sum(&sc)?)},
  "Sqrt" => {
    let inner = Expr(&sc)?;
    Ok(Expr::sqrt(inner))
  },
  "Prod" => {Ok(prod(&sc)?)},
  //       ^other chars that could be numerical
]
```
That becomes:
```rust
/*-P->assert(rod)----->*/{Ok(prod(&sc)?)}
/*  \-S-u->assert(m)-->*/{Ok(sum(&sc)?)}
/*     \-q->assert(rt)>*/{
                           let inner = Expr(&sc)?;
                            Ok(Expr::sqrt(inner))
                         },
```


## Copy-able Expr

### The problem

```rust
// Having to call \/ `.clone()` \/ here
assert_eq!(e_a.clone() * e_b.clone(), Expr::one());
// bc we also \/ mul here
assert_eq!(e_b * e_a, Expr::one());
```

### ramblings
Lifetime chart

```
f: Field
  |\-π         // let pi = f.c_pi()
  |<-| f: [π]
  |  |
  |  |\-Σ(2,π) // let k = 2*pi
  |<-----| f: [π,2π]
  |  |   |     |  \-Newly added
  |  |   |     |\<--/ references π
  |  |   |     \-Can never be removed as long as 2π exists
  |  |   |       We don't know how many '2π's exist
  |  |   |       bc π, 2π can be arbitrarily copied (Bitwise)
  |  |   |
  |  |   |   /---Static field exists forever.
-------------\   Using it is safe
  |  |  /     \--Below this line: Disposable field: !unsafe!
----------2π dies. 
  |  |    We are very very sure that 2π is not copied anywhere
  |  /
  | /
-------- π dies.
  |      We are very very sure that π is really dead
```
Actually, constants should be an enum or expand the following into Expr:
```rust
Enum Const {
  Known{
    ch: char,
  },
  Expr{
    ch: char,      // 'φ'
    ascii: String, // "phi"<- does String implement Copy?
    asc_ch: char,  // 'f'
    expr: String,   //  Σ[(1/2,ξ1),(1/2,Π(ξ5,1/2))]
  }
}

const ConstKnownStruct {
// ch  ascii  asc_ch   expr    eval
  ('π', "pi" , 'p', "Σ[???]", std::f64::consts::PI),
  ('e', "e"  , 'e', "Σ[1/n!]" std::f64::consts::E ),

  // \/ ?not? in this list
  ('φ', "phi", 'f', "Σ1[5^(n/2)/2]" = "Qout(Sum(1+Sqrt5)/2)", (1.0+5.0.sqrt)/2),
  // /\ bc it *it* used a lot in polytope
  // Actually include all of Klitzing's list here
  ...
}
```
What to do if the expression is indeterminate?

memory layout diagram
```rust
/*General                                                                */
/*|Expression |                                                          */
/*|-----------|                                                          */
/*|field|ptr  |----->|Field     |                                        */
/*|index|usize|      |----------|                                        */
/*|expr |enum |      |exprs| vec|                                        */
/*|-----|-----|      |-----|----|                                        */
/*                                                                       */
/*Example     : π * 2                                                    */
/*{:p} anything                                                          */
/*|pi:Expression|                                                        */
/*|-------------|      {:p} 0xfed                                        */
/*|field| 0xfed |----->|f: Field  |                                      */
/*|index| 0     |      |----------| /-This is probably a different π     */
/*|expr | 'π'   |      |exprs| vec[π] The important part is that         */
/*|-----|-------|      |-----|----|   Field is reachable from *any* π    */
/*      |                  |                                             */
/*----------------------------\*/ let pi_2 = 2 * pi; /* 2 is type F ----\*/
/*      |                  |  \*/            = pi * 2;/*                |*/
/*/f\<- |                <-/                 \- invert so lhs is expr<=-/*/
/*\|/ /-|--\                                    Mul is impl'd for Expr   */
/* | |Σ(2,π)|                                                            */
/* |  \-|--/                                                             */
/* |<---|*/ return self.field.add_raw(Sum{terms:vec![2,pi]}); /*<-ab     */
/*||-----|----------|                                                    */
/*|exprs |vec![π]   | <- 2π not in there -\                              */
/*|exprs |vec![π,2π]|<--add it<=----------/                              */
/*||-------------/--|                                                    */
/* | |pi_2:Expression|*/exprs.push(Expr::new{ /*                         */
/* | |---------/-----|*/  field: &self,       /*                         */
/* | |expr | '2π'    |*/  index: exprs.len(), /*                         */
/* | |field| 0xfed   |*/})                    /*                         */
/* | |index| 1       |                                                   */
/* | |-----|---------|  return exprs.last()                              */
/* |f   |π |2π                                                           */
/* |\exprs.last                                                          */
/* |  \-|\ab                                                             */
/* |    |  \--------------------/\                                       */
```

### Solution
Implement a copyable Expr type using Field as factory:

- [ ] Expr:
  ```rust
  struct Expr<FieldType> {
    field{ field_vec: <some pointer type that implements COPY and points directly to Field>,
    field_index: usize }
    expr: ExprEnum
  }

  or 

  enum FieldRef {
    Rc(Rc),
    FlatField(Vec<Expr>)
    TopField
  }

  struct FieldRef {
    field: 
    data: 
  }
  
  impl Deref,DerefMut for FieldRef {

  }


  Enum Expr<FieldRef: TraitsOfRc + Copy> {
    Zero(FieldRef<()>) // or Expr
    One(FieldRef<()>)
    Val(FieldRef<F>)
    Sum(FieldRef<Sum>)
  }
  ```
  The problem is not in accessing the data, but when a new expression is created and added to the vector, it may be a race condition or something.
  - Do vectors move the data when stuff is added?
  - other questions about unsafe code: is it needed at all?
  - Just do not allow the FieldArray to shrink
    - ?Create disposable FlatFields in operations?:
      ```rust
      fn do_stuff(expr: Expr<T>) {
        let f_dis = FlatField::new();     // \__ todo: put this in fn
        let k = f_dis.sum_pi_i(2) * expr; // /   [π,2π]
        //       \- ops adds to the lhs field
        if (k+'π') == (k-'π')             //     [π,2π,3π]
        //  \|--/     \-|--/
        {} else if (k.inv()) != f.parse("2/π")//[π,2π,3π,1/2π,2/π]
        //   |      \---|-/             \----|
        //   \--these all add to dis_field---/
        {
          return expr * k; //ops add to the lhs field. Thus, f_dis goes out of scope and is destroyed
        }
      ```
      not needed when operations directly work on clones (?copies?) of the vector:
      ```rust
      // Σ*Σ
      let f: FieldRef<T> = self.field_ref;
      let res: Svec = mul_sum(self.terms, rhs.terms)
      f.add(res)
      ```
- [ ] FlatBuffer-type:
  ```rust
  use crate::<tree>::ExprFieldFlat as Field
  let f: Field = Field::new;
  let s = f.parse("Σ(2,π)")''
  field: { 
    members: vec!['π','Σ(2,π)']
  };
  ```
- [ ] Topological-ness-type:
  ```rust
  use crate::<tree>::ExprFieldTop as Field
  let f: Field = Field::new();
  let s = f.parse("Σ(2,π)");
  assert_eq!(field, Field{
    sums: vec!['Σ(2,π)'] //<- these can be emptied
    prods: []            //<- if there is nothing in composites
    fns: []
    composites: []
    constants: vec!['π']
  });
  let rt = s.sqrt();
  assert_eq!(field, Field{
    sums: vec!['Σ(2,π)']
    prods: []
    composites: []
    constants: vec!['π']
  });
  ```
- [ ] 

## Storing expr and ref always together is inefficient

Not this:
```rust
enum Expr{
  Sum(Ref, SVec)
}

struct FlatField{
  exprs: Vec<Expr>
}
```
but this
```rust
enum Expr{
  Sum(Ref),
  Prod(Ref),
}

struct TypeField{
  vals: Vec<F>,
  consts: Vec<Const>,
  sums: Vec<SVec>,
  prods: Vec<PVec>,
  fns: Vec<ExprFn>,
  // ...
}
```
or this
```rust
enum Expr{
  Sum(SVec)
}
```