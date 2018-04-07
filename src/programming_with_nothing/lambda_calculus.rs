extern crate computationbook;

use computationbook::programming_with_nothing::lambda::lambda::{Lambda};
use computationbook::programming_with_nothing::lambda::reduce::{Reduce};

fn print_assert(expr: &Box<Lambda>, name: &str, ans: &str) {
    let fmtstr = format!("{}", expr);
    println!("{}: {}", name, fmtstr);
    assert_eq!(ans, fmtstr);
}

pub fn main() {
    let one = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x"))));
    let increment = Lambda::lcfun("n",
                      Lambda::lcfun("p",
                        Lambda::lcfun("x",
                          Lambda::lccall(
                            Lambda::lcvar("p"),
                            Lambda::lccall(
                              Lambda::lccall(Lambda::lcvar("n"), Lambda::lcvar("p")),
                              Lambda::lcvar("x")
                            )
                          )
                        )
                      )
                    );
    let add = Lambda::lcfun("m", Lambda::lcfun("n", Lambda::lccall(Lambda::lccall(Lambda::lcvar("n"), increment.clone()), Lambda::lcvar("m"))));

    print_assert(&one, "one", "-> p { -> x { p[x] } }");
    print_assert(&increment, "increment", "-> n { -> p { -> x { p[n[p][x]] } } }");
    print_assert(&add, "add", "-> m { -> n { n[-> n { -> p { -> x { p[n[p][x]] } } }][m] } }");

    let mut expr = Lambda::lcvar("x");
    print_assert(&expr, "var x", "x");
    print_assert(&expr.replace("x", &Lambda::lcfun("y", Lambda::lcvar("y"))), "replace x with y -> { y }", "-> y { y }");
    print_assert(&expr.replace("z", &Lambda::lcfun("y", Lambda::lcvar("y"))), "replace z with y -> { y }", "x");

    expr = Lambda::lccall(
            Lambda::lccall(
                Lambda::lccall(
                    Lambda::lcvar("a"),
                    Lambda::lcvar("b")),
                Lambda::lcvar("c")),
            Lambda::lcvar("b"));
    println!("expression: {}", expr);
    print_assert(&expr.replace("a", &Lambda::lcvar("x")), "replace a with x", "x[b][c][b]");
    print_assert(&expr.replace("b", &Lambda::lcfun("x", Lambda::lcvar("x"))),
                 "replace b with x -> {x}", "a[-> x { x }][c][-> x { x }]");

    expr = Lambda::lcfun("y", Lambda::lccall(
            Lambda::lcvar("x"),
            Lambda::lcvar("y")));
    println!("expression: {}", expr);
    print_assert(&expr.replace("x", &Lambda::lcvar("z")), "replace x with z", "-> y { z[y] }");
    print_assert(&expr.replace("y", &Lambda::lcvar("z")), "replace y with z", "-> y { x[y] }");

    expr = Lambda::lccall(
        Lambda::lccall(Lambda::lcvar("x"), Lambda::lcvar("y")),
        Lambda::lcfun("y", Lambda::lccall(Lambda::lcvar("y"), Lambda::lcvar("x")))
    );

    println!("expression: {}", expr);
    print_assert(&expr.replace("x", &Lambda::lcvar("z")), "replace x with z", "z[y][-> y { y[z] }]");
    print_assert(&expr.replace("y", &Lambda::lcvar("z")), "replace x with z", "x[z][-> y { y[x] }]");

    let func = Lambda::lcfun("x", Lambda::lcfun("y", Lambda::lccall(Lambda::lcvar("x"), Lambda::lcvar("y"))));
    let arg = Lambda::lcfun("z", Lambda::lcvar("z"));
    println!("func: {}", func);
    println!("arg: {}", arg);
    print_assert(&func.call(&arg), "call func with arg", "-> y { -> z { z }[y] }");

    expr = Lambda::lccall(Lambda::lccall(add, one.clone()), one.clone());
    while expr.reducible() {
        println!("    {}", expr);
        expr = expr.reduce()
    }
    print_assert(&expr, "add[one][one]", "-> p { -> x { p[-> p { -> x { p[x] } }[p][x]] } }");

    let inc = Lambda::lcvar("inc");
    let zero = Lambda::lcvar("zero");

    expr = Lambda::lccall(Lambda::lccall(expr, inc), zero);
    while expr.reducible() {
        println!("    {}", expr);
        expr = expr.reduce()
    }
    print_assert(&expr, "add[one][one] on inc and zero", "inc[inc[zero]]");
}
