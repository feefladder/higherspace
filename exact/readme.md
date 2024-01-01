# Exact arithmetic crate

This crate provides exact arithmetic data structures to be used as drop-in replacements for `f64` when the expressions to be used are known to be elegant algebraically. It also provides a parser. Currently implemented:

* Value (fraction)
* Const

What should actually be done:

Only ever work with Rc's to the data. Maybe store them in a HashSet for locality.