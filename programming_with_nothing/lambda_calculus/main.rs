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
}
