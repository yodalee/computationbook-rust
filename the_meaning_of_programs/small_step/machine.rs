use syntax::Node;

pub struct Machine {
    expression: Box<Node>,
}

impl Machine {
    pub fn new(expression: Box<Node>) -> Machine {
        Machine{ expression: expression }
    }

    pub fn step(&mut self) {
        self.expression = self.expression.reduce();
    }

    pub fn run(&mut self) {
        while self.expression.reducible() {
            println!("{}", self.expression);
            self.step();
        }
        println!("{}", self.expression);
    }
}
