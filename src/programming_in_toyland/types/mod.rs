pub mod syntaxtype;
pub mod context;

#[cfg(test)]
mod tests {
    use the_meaning_of_programs::simple::syntax::{Node};
    use the_meaning_of_programs::simple::evaluate::{Evaluate};
    use the_meaning_of_programs::simple::environment::{Environment};
    use super::syntaxtype::{SyntaxType, Type};
    use super::context::{Context};

    #[test]
    fn test_types_basic() {
        let mut context = Context::new();
        assert_eq!("3", format!("{}", Node::add(Node::number(1), Node::number(2)).evaluate(&mut Environment::new())));
        assert_eq!(Some(Type::Number), Node::add(Node::number(1), Node::number(2)).get_type(&mut context));
        assert_eq!(None, Node::add(Node::number(1), Node::boolean(true)).get_type(&mut context));
        assert_eq!(Some(Type::Boolean), Node::lessthan(Node::number(1), Node::number(2)).get_type(&mut context));
        assert_eq!(None, Node::lessthan(Node::number(1), Node::boolean(true)).get_type(&mut context));
    }

    #[test]
    #[should_panic]
    fn test_types_blowup() {
        // blow up
        println!("{}", Node::add(Node::number(1), Node::boolean(true)).evaluate(&mut Environment::new()));
    }

    #[test]
    fn test_types_variable() {
        let mut context = Context::new();
        let expr = Node::add(Node::variable("x"), Node::variable("y"));
        assert_eq!(None, expr.get_type(&mut context));
        context.add("x", Type::Number);
        context.add("y", Type::Number);
        assert_eq!(Some(Type::Number), expr.get_type(&mut context));

        context.add("y", Type::Boolean);
        assert_eq!(None, expr.get_type(&mut context));
    }

    #[test]
    fn test_types_statement() {
        let mut context = Context::new();
        assert_eq!(Some(Type::Void),
                   Node::If(Node::lessthan(Node::number(1), Node::number(2)),
                            Node::donothing(),
                            Node::donothing()).get_type(&mut context));
        assert_eq!(None,
                   Node::If(Node::add(Node::number(1), Node::number(2)),
                            Node::donothing(),
                            Node::donothing()).get_type(&mut context));

        context.add("x", Type::Boolean);

        assert_eq!(Some(Type::Void),
                   Node::while_node(Node::variable("x"),
                                    Node::donothing()).get_type(&mut context));
        context.add("x", Type::Number);
        assert_eq!(None,
                   Node::while_node(Node::variable("x"),
                                    Node::donothing()).get_type(&mut context));
    }

    #[test]
    fn test_assign() {
        let stat = Node::while_node(
            Node::lessthan(Node::variable("x"), Node::number(5)),
            Node::assign("x", Node::add(Node::variable("x"), Node::number(3))));
        println!("{}", stat);

        let mut context = Context::new();
        assert_eq!(None, stat.get_type(&mut context));
        context.add("x", Type::Number);
        assert_eq!(Some(Type::Void), stat.get_type(&mut context));
        context.add("x", Type::Boolean);
        assert_eq!(None, stat.get_type(&mut context));
    }

    #[test]
    fn test_infiniteloop() {
        let stat = Node::sequence(
            Node::assign("x", Node::number(0)),
            Node::while_node(Node::boolean(true),
                             Node::assign("x", Node::add(Node::variable("x"), Node::number(1)))));
        let mut context = Context::new();
        context.add("x", Type::Number);

        println!("{}", stat);
        assert_eq!(Some(Type::Void), stat.get_type(&mut context));

        //blow up
        //println!("{}", stat.evaluate(&mut Environment::new()));

        context.add("x", Type::Number);
        assert_eq!(None, Node::sequence(stat, Node::assign("x", Node::boolean(true))).get_type(&mut context));
    }

    #[test]
    fn test_types_static_undetermined() {
        // type of x depend on variable b
        let stat = Node::sequence(
            Node::if_cond_else(Node::variable("b"),
                               Node::assign("x", Node::number(6)),
                               Node::assign("x", Node::boolean(true))),
            Node::sequence(
                Node::if_cond_else(Node::variable("b"),
                                   Node::assign("y", Node::variable("x")),
                                   Node::assign("y", Node::number(1))),
                Node::assign("z", Node::add(Node::variable("y"), Node::number(1)))));
        println!("{}", stat);

        let mut environment = Environment::new();
        environment.add("b", Node::boolean(true));
        stat.evaluate(&mut environment);
        assert_eq!(Box::new(Node::Boolean(true)), environment.get("b"));
        assert_eq!(Box::new(Node::Number(6)), environment.get("x"));
        assert_eq!(Box::new(Node::Number(6)), environment.get("y"));
        assert_eq!(Box::new(Node::Number(7)), environment.get("z"));

        environment = Environment::new();
        environment.add("b", Node::boolean(false));

        stat.evaluate(&mut environment);
        assert_eq!(Box::new(Node::Boolean(false)), environment.get("b"));
        assert_eq!(Box::new(Node::Boolean(true)), environment.get("x"));
        assert_eq!(Box::new(Node::Number(1)), environment.get("y"));
        assert_eq!(Box::new(Node::Number(2)), environment.get("z"));

        let mut context = Context::new();
        context.add("b", Type::Boolean);
        context.add("y", Type::Number);
        context.add("z", Type::Number);

        assert_eq!(None, stat.get_type(&mut context));
        context.add("x", Type::Number);
        assert_eq!(None, stat.get_type(&mut context));
        context.add("x", Type::Boolean);
        assert_eq!(None, stat.get_type(&mut context));
    }

    #[test]
    fn test_types_static_unchecked() {
        let stat = Node::assign("x", Node::add(Node::variable("x"), Node::number(1)));
        let mut context = Context::new();
        context.add("x", Type::Number);
        stat.get_type(&mut context);
        //environment = Environment::new();
        //blow up stat.evaluate(&mut environment);
    }
}
