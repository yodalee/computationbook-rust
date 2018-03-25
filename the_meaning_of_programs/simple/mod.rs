pub mod machine;
pub mod syntax;
pub mod environment;
pub mod reduce;
pub mod evaluate;
pub mod denotational;

#[cfg(test)]
mod tests {
    pub use ::*;

    #[test]
    fn test_simple_small_number() {
        let n = Node::number(3);
        assert_eq!("3", format!("{}", n));
        assert!(!n.reducible());
    }

    #[test]
    fn test_simple_small_arithmetic() {
        let m = Node::add(
            Node::multiply(Node::number(1), Node::number(2)),
            Node::multiply(Node::number(3), Node::number(4)));
        assert!(m.reducible());
        let mut machine = Machine::new_with_empty_env(m);
        machine.run();
        assert!(!machine.get_expression().reducible());
        assert_eq!(14, machine.get_expression().value());
    }

    #[test]
    fn test_simple_small_lessthan() {
        let m = Node::lessthan(Node::number(5), Node::add(Node::number(2), Node::number(2)));
        let mut machine = Machine::new_with_empty_env(m);
        machine.run();
        assert!(!machine.get_expression().condition());
    }

    #[test]
    fn test_simple_small_variable() {
        let mut env = Environment::new();
        env.add("x", Node::number(3));
        env.add("y", Node::number(4));
        let mut machine = Machine::new(Node::add(Node::variable("x"), Node::variable("y")), env);
        machine.run();

        assert_eq!(7, machine.get_expression().value());
    }

    #[test]
    fn test_simple_small_statement() {
        let mut statement = Node::assign("x", Node::add(Node::variable("x"), Node::number(1)));
        let mut env = Environment::new();
        env.add("x", Node::number(2));

        assert!(statement.reducible());
        statement = statement.reduce(&mut env);
        println!("{0}; {1}", statement, env);
        statement = statement.reduce(&mut env);
        println!("{0}; {1}", statement, env);
        statement = statement.reduce(&mut env);
        println!("{0}; {1}", statement, env);
        assert!(!statement.reducible());
    }

    #[test]
    fn test_simple_small_true() {
        let mut env = Environment::new();
        env.add("x", Node::boolean(true));

        let mut machine = Machine::new(
            Node::if_cond_else(
                Node::variable("x"),
                Node::assign("y", Node::number(1)),
                Node::assign("y", Node::number(2))
            ), env
        );
        machine.run();
        assert_eq!(1, machine.environment.get("y").value());
    }

    #[test]
    #[should_panic]
    fn test_simple_small_false() {
        let mut env = Environment::new();
        env.add("x", Node::boolean(false));
        let mut machine = Machine::new(
            Node::if_cond_else(
                Node::variable("x"),
                Node::assign("y", Node::number(1)),
                Node::donothing()
            ), env
        );
        machine.run();
        assert!(machine.environment.get("y").condition()); // should blow up
    }

    #[test]
    fn test_simple_small_sequence() {
        let mut machine = Machine::new_with_empty_env(
            Node::sequence(
                Node::assign("x", Node::add(Node::number(1), Node::number(1))),
                Node::assign("y", Node::add(Node::variable("x"), Node::number(3))),
            )
        );
        machine.run();
        assert_eq!(2, machine.environment.get("x").value());
        assert_eq!(5, machine.environment.get("y").value());
    }

    #[test]
    fn test_simple_small_while() {
        let mut env = Environment::new();
        env.add("x", Node::number(1));
        let mut machine = Machine::new(
            Node::while_node(
                Node::lessthan(Node::variable("x"), Node::number(5)),
                Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3)))
            ), env
        );

        machine.run();
        assert_eq!(9, machine.environment.get("x").value());
    }

    #[test]
    fn test_simple_big_number() {
        let n = Node::number(3);
        let mut env = Environment::new();
        assert_eq!(3, n.evaluate(&mut env).value());
    }

    #[test]
    fn test_simple_big_variable() {
        let n = Node::variable("x");
        let mut env = Environment::new();
        env.add("x", Node::number(23));
        assert_eq!(23, n.evaluate(&mut env).value());
    }

    #[test]
    fn test_simple_big_arithmetic() {
        let n = Node::multiply(Node::number(14), Node::number(3));
        let mut env = Environment::new();
        assert_eq!(42, n.evaluate(&mut env).value());
    }

    #[test]
    fn test_simple_big_lessthan() {
        let n = Node::lessthan(Node::add(Node::variable("x"), Node::number(2)), Node::variable("y"));
        let mut env = Environment::new();
        env.add("x", Node::number(2));
        env.add("y", Node::number(5));
        assert!(n.evaluate(&mut env).condition());
    }

    #[test]
    fn test_simple_big_sequence() {
        let statement = Node::sequence(
            Node::assign("x", Node::add(Node::number(1), Node::number(1))),
            Node::assign("y", Node::add(Node::variable("x"), Node::number(3)))
        );
        let mut env = Environment::new();
        println!("{}", statement.evaluate(&mut env));
        assert_eq!(2, env.get("x").value());
        assert_eq!(5, env.get("y").value());
    }

    #[test]
    fn test_simple_big_while() {
        let statement = Node::while_node(
            Node::lessthan(Node::variable("x"), Node::number(5)),
            Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3))),
        );
        let mut env = Environment::new();
        env.add("x", Node::number(1));
        println!("{}", statement.evaluate(&mut env));
        assert_eq!(9, env.get("x").value());
    }

    #[test]
    fn test_simple_toruby() {
        assert_eq!("-> e { 5 }", Node::number(5).to_ruby());
        assert_eq!("-> e { false }", Node::boolean(false).to_ruby());
        let expr = Node::variable("x");
        assert_eq!("-> e { e[:x] }", expr.to_ruby());
        assert_eq!("-> e { (-> e { e[:x] }).call(e) + (-> e { 1 }).call(e) }", Node::add(Node::variable("x"), Node::number(1)).to_ruby());

        assert_eq!("-> e { (-> e { (-> e { e[:x] }).call(e) + (-> e { 1 }).call(e) }).call(e) < (-> e { 3 }).call(e) }",
            Node::lessthan(Node::add(Node::variable("x"), Node::number(1)), Node::number(3)).to_ruby());
        assert_eq!("-> e { e.merge({ :y => (-> e { (-> e { e[:x] }).call(e) + (-> e { 1 }).call(e) }).call(e) }) }",
            Node::assign("y", Node::add(Node::variable("x"), Node::number(1))).to_ruby());
        assert_eq!("-> e { while (-> e { (-> e { e[:x] }).call(e) < (-> e { 5 }).call(e) }).call(e); e = (-> e { e.merge({ :x => (-> e { (-> e { e[:x] }).call(e) * (-> e { 3 }).call(e) }).call(e) }) }).call(e); end; e }",
            Node::while_node(
                Node::lessthan(Node::variable("x"), Node::number(5)),
                Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3)))).to_ruby());
    }
}
