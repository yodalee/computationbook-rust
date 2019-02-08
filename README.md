Understanding Computation example code in Rust
==============================================

The example code of [Understanding Computation](http://computationbook.com/), an O’Reilly book about computation theory.
Re-implement by language [Rust](https://github.com/rust-lang/rust).

# Build:
Use
```
cargo build
```
to build the code.

# Table of Content:

* Chapter 2: [The Meaning of Programs](src/the_meaning_of_programs)
    * [syntax](src/the_meaning_of_programs/simple/syntax.rs)
    * [small-step operational semantics](src/the_meaning_of_programs/simple/reduce.rs)
    * [big-step operational semantics](src/the_meaning_of_programs/simple/evaluate.rs)
    * [denotational semantics](src/the_meaning_of_programs/simple/denotational.rs)
    * [simple-parser](src/the_meaning_of_programs/simple-parser.rs)
* Chapter 3: [The Simplest Computers](src/the_simplest_computers)
    * [deterministic and nondeterministic finite automata](src/the_simplest_computers/finite_automata)
    * [regular expressions](src/the_simplest_computers/regular_expressions)
    * [regex parser](src/the_simplest_computers/regex-parser.rs) 
* Chapter 4: [Just Add Power](src/just_add_power)
    * [deterministic and non-deterministic pushdown automata](src/just_add_power)
* Chapter 5: [The Ultimate Machine](src/the_ultimate_machine)
    * [deterministic Turing machines](src/the_ultimate_machine)
* Chapter 6: [Programming with Nothing](src/programming_with_nothing)
    * [FizzBuzz with Closures](src/programming_with_nothing/fizzbuzz.rs)
    * [the lambda calculus](src/programming_with_nothing/lambda_calculus.rs)
    * [lambda parser](src/programming_with_nothing/lambda-parser.rs)
* Chapter 7: [Universality is Everywhere](src/universality_is_everywhere)
    * [partial recursive function](src/universality_is_everywhere/partial_recursive_function)
    * [ski calculus](src/universality_is_everywhere/ski_calculus)
    * [iota calculus](src/universality_is_everywhere/iota)
    * [tag systems](src/universality_is_everywhere/tag_systems)
    * [cyclic tag systems](src/universality_is_everywhere/cyclic_tag_systems)
* Chapter 8: Not planning to implement this chapter.
* Chapter 9: [Programming in Toyland](src/programming_in_toyland)
    * [abstract interpretation: arithmetic with signs](src/programming_in_toyland/signs)
    * [type systems](src/programming_in_toyland/types)

# Reading guide:

Most of the example code is implemented in testing form, you can find them in mod.rs.
To view the test result, use following command:
```
RUST_TEST_THREADS=1 cargo test -- --nocapture
```
You can specify keyword in testing name in the command line.  

For example:
```
$ RUST_TEST_THREADS=1 cargo test -- --nocapture ski_swap
running 1 test
test universality_is_everywhere::ski_calculus::tests::test_ski_swap ... swap: S[K[S[I]]][K]
S[K[S[I]]][K][x][y]
K[S[I]][x][K[x]][y]
S[I][K[x]][y]
I[y][K[x][y]]
y[K[x][y]]
y[x]
ok
```

If you find any bugs or other programs with the code, please [open an issue](https://github.com/yodalee/computationbook-rust/issues/new).

