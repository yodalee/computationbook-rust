mod lambda;

use lambda::Lambda;

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
}
