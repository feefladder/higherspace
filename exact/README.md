# Exact arithmetic crate

⚠️under heavy development⚠️

This crate provides exact arithmetic data structures to be used as drop-in replacements for `f64` when the expressions to be used are known to be elegant algebraically. Initially developed to solve a problem for matrix multiplications involving square roots √ where after a number of multiplications the floating-point error rose. There are a few flavours, each with trade-offs:

## Expr Rc

- no `Copy` trait

This uses `Rc`s for keeping references. As such, it doesn't implement the `Copy` trait and is thus not a drop-in replacement for floats. However, it will not blow up your memory as badly if you were to run e.g. an erosion model.

## Expr Field

- Field as Factory constructor
- no `One` and `Zero` traits
- fills your memory if not used carefully
- additional level of indirection

Uses "Field" in the [mathematical sense](https://en.wikipedia.org/wiki/Field_(mathematics)):

> By a field we will mean every infinite system of real or complex numbers so closed in itself and perfect that addition, subtraction, multiplication, and division of any two of these numbers again yields a number of the system.
> — Richard Dedekind, 1871

That is, for every operation that is applied to two numbers in a field, the result is added to that field. An operation to two numbers in different fields will add the result to the left-hand-side of the operation:

```rust
use exact::expr_field::TypeField;
let f1 = TypeField::new();
let f2 = TypeField::new();

let phi1 = f1.parse("Σ[1,√5]")
let phi2 = f2.parse("Σ[1,√5]")

let phi_squared = phi1 * phi2;

assert!(std::ptr::eq(phi_squared.field(),f1))
assert_eq!(phi_squared, f1.parse("Σ[6,3√5]"))
```


## Static Field

- not yet implemented
- `One` and `Zero` traits

uses a statically available Field that is the default field.

# parsing and printing

Parsing and typing is subject to change. Ideally, we'd be able to parse and print expressions in complete human-readable form: `1+√5` This will undergo the following changes:

### certain:
```
current:                    `Σ[(1,Ι),(1,√5)]` `Π[(π,2),(e,1)]`
omit redundant `1`s:        `Σ[1,√5]`         `Π[π²,e]`
replace `,` by `+`/"":      `Σ[1+√5]`         `Π[π²e]`
```
### maybe:
```
remove `Σ`/'Π' sign:         `[1+√5]`          `[π²e]`
remove delimiters             `1+√5`            `π²e`
```

tips for easy typing
- use a Greek keyboard layout:
  - 'Σ' : 'S'
  - 'Π': 'P'
  - 'π': 'p'
  - 'ξ': 'j'
- use an environment with unicode input plugins/support

tips for nice display:
- Use JuliaMono font