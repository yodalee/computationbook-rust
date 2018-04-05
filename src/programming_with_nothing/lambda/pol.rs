use std::rc::Rc;

pub enum Pol {
    C(Rc<Fn(Rp) -> Rp>),
    I(i32),
    B(bool),
}
pub type Rp = Rc<Pol>;

#[macro_export]
macro_rules! r {
    ($cl:expr) => {Rc::new(Pol::C(Rc::new($cl)))}
}

impl Pol {
    pub fn call(&self, x: Rp) -> Rp {
        match self {
            &Pol::C(ref c) => c(x),
            _ => panic!(),
        }
    }
}
