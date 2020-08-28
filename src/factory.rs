use crate::variable;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::marker::PhantomData;

pub struct Factory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
    pub model: String,
    pub sequence: Cell<u16>,
    pub gen_func: F,
    _maker: PhantomData<&'a T>,
}

pub fn new<'a, T, S>(model: T, suite: S) -> Factory<'a, S, T>
where
    T: Serialize + Deserialize<'a>,
    S: Fn(&mut T, u16),
{
    let factory = Factory {
        model: serde_json::to_string(&model).unwrap(),
        sequence: Cell::new(1),
        gen_func: suite,
        _maker: PhantomData,
    };
    factory
}

pub fn sequence(from: u16, n: u16) -> u16 {
    from + n - 1
}

pub fn sequence_a(from: &str, n: u16) -> String {
    variable::ALPHABET
        [(*variable::ALPHABET_INDEX.get(from).unwrap() as usize + (n - 1) as usize) % 25]
        .to_string()
}

impl<'a, F, T> Factory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
    pub fn build(&'a self) -> T {
        let mut model = serde_json::from_str(self.model.as_str()).unwrap();
        let suite = &self.gen_func;
        suite(&mut model, self.sequence.get());
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
