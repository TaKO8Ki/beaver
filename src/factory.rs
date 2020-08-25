use serde::{Deserialize, Serialize};
use std::cell::Cell;

pub struct Factory<T> {
    pub a: String,
    pub count: Cell<u16>,
    pub gen_func: Vec<GenFunc<fn(&mut T, u16)>>,
}

pub enum GenFunc<F> {
    Sequence(F),
}

pub fn new<'a, T>(s: T) -> Factory<T>
where
    T: Serialize + Deserialize<'a>,
{
    Factory {
        a: serde_json::to_string(&s).unwrap(),
        count: Cell::new(1),
        gen_func: vec![],
    }
}

impl<'a, T> Factory<T>
where
    T: Serialize + Deserialize<'a>,
{
    pub fn sequence(&mut self, f: fn(&mut T, u16)) -> &mut Self {
        self.gen_func.push(GenFunc::Sequence(f));
        self
    }

    pub fn create(&'a self) -> T {
        let mut model = serde_json::from_str(self.a.as_str()).unwrap();
        for f in &self.gen_func {
            match f {
                GenFunc::Sequence(f) => f(&mut model, self.count.get()),
            }
        }
        self.count.set(self.count.get() + 1);
        model
    }

    pub fn create_list(&'a self, number: u16) -> Vec<T> {
        let mut list = vec![];
        for _ in 0..number {
            list.push(self.create())
        }
        list
    }
}
