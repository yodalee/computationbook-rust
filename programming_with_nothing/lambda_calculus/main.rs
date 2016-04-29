mod lambda;

use lambda::{Lambda, Reduce};

pub fn main() {
    let one = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x"))));
    println!("{}", one);
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
    println!("{}", increment);

    let add = Lambda::lcfun("m", Lambda::lcfun("n", Lambda::lccall(Lambda::lccall(Lambda::lcvar("n"), increment), Lambda::lcvar("m"))));
    println!("{}", add);

    let mut expr = Lambda::lcvar("x");
    println!("{}", expr.replace("x", &Lambda::lcfun("y", Lambda::lcvar("y"))));
    println!("{}", expr.replace("z", &Lambda::lcfun("y", Lambda::lcvar("y"))));

    expr = Lambda::lccall(
            Lambda::lccall(
                Lambda::lccall(
                    Lambda::lcvar("a"),
                    Lambda::lcvar("b")),
                Lambda::lcvar("c")),
            Lambda::lcvar("b"));
    println!("{}", expr.replace("a", &Lambda::lcvar("x")));
    println!("{}", expr.replace("b", &Lambda::lcfun("x", Lambda::lcvar("x"))));

    expr = Lambda::lcfun("y", Lambda::lccall(Lambda::lcvar("x"), Lambda::lcvar("y")));
    println!("{}", expr.replace("x", &Lambda::lcvar("z")));
    println!("{}", expr.replace("y", &Lambda::lcvar("z")));

    expr = Lambda::lccall(
        Lambda::lccall(Lambda::lcvar("x"), Lambda::lcvar("y")),
        Lambda::lcfun("y", Lambda::lccall(Lambda::lcvar("y"), Lambda::lcvar("x")))
    );

    println!("{}", expr.replace("x", &Lambda::lcvar("z")));
    println!("{}", expr.replace("y", &Lambda::lcvar("z")));

    let func = Lambda::lcfun("x", Lambda::lcfun("y", Lambda::lccall(Lambda::lcvar("x"), Lambda::lcvar("y"))));
    let arg = Lambda::lcfun("z", Lambda::lcvar("z"));
    println!("{}", func.call(&arg));

    expr = Lambda::lccall(Lambda::lccall(add, one.clone()), one.clone());
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);

    let inc = Lambda::lcvar("inc");
    let zero = Lambda::lcvar("zero");

    expr = Lambda::lccall(Lambda::lccall(expr, inc), zero);
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);
}
