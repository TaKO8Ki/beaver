use serde::{Deserialize, Serialize};
use std::cell::Cell;

pub struct Factory<T> {
    pub model: String,
    pub sequence: Cell<u16>,
    pub gen_func: Vec<GenFunc<T>>,
}

pub enum GenFunc<T> {
    Sequence(fn(&mut T, u16)),
    Attribute(fn(&mut T)),
    SubFactory(Box<dyn Fn(&mut T)>),
}

pub fn new<'a, T, S>(model: T, suite: S) -> Factory<T>
where
    T: Serialize + Deserialize<'a>,
    S: Fn(&mut Factory<T>) -> (),
{
    let mut factory = Factory {
        model: serde_json::to_string(&model).unwrap(),
        sequence: Cell::new(1),
        gen_func: vec![],
    };
    suite(&mut factory);
    factory
}

impl<'a, T> Factory<T>
where
    T: Serialize + Deserialize<'a>,
{
    pub fn sequence(&mut self, from: u16, f: fn(&mut T, u16)) -> &mut Self {
        self.sequence.set(from);
        self.gen_func.push(GenFunc::Sequence(f));
        self
    }

    pub fn attribute(&mut self, f: fn(&mut T)) -> &mut Self {
        self.gen_func.push(GenFunc::Attribute(f));
        self
    }

    pub fn sub_factory(&mut self, f: Box<dyn Fn(&mut T)>) -> &mut Self {
        self.gen_func.push(GenFunc::SubFactory(f));
        self
    }

    pub fn build(&'a self) -> T {
        let mut model = serde_json::from_str(self.model.as_str()).unwrap();
        for f in &self.gen_func {
            match f {
                GenFunc::Sequence(f) => f(&mut model, self.sequence.get()),
                GenFunc::Attribute(f) => f(&mut model),
                GenFunc::SubFactory(f) => f(&mut model),
            }
        }
        self.sequence.set(self.sequence.get() + 1);
        model
    }

    pub fn build_list(&'a self, number: u16) -> Vec<T> {
        let mut list = vec![];
        for _ in 0..number {
            list.push(self.build())
        }
        list
    }
}
