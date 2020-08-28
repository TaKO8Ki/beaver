use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::marker::PhantomData;

pub struct FaFactory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
    pub model: String,
    pub sequence: Cell<u16>,
    pub gen_func: F,
    _maker: PhantomData<&'a T>,
}

pub fn nnew<'a, T, S>(model: T, suite: S) -> FaFactory<'a, S, T>
where
    T: Serialize + Deserialize<'a>,
    S: Fn(&mut T, u16),
{
    let factory = FaFactory {
        model: serde_json::to_string(&model).unwrap(),
        sequence: Cell::new(1),
        gen_func: suite,
        _maker: PhantomData,
    };
    factory
}

pub fn sequence(from: u16, n: u16) -> u16 {
    from + n
}

impl<'a, F, T> FaFactory<'a, F, T>
where
    T: Serialize + Deserialize<'a>,
    F: Fn(&mut T, u16),
{
    pub fn build(&'a self) -> T {
        let mut model = serde_json::from_str(self.model.as_str()).unwrap();
        let hoge = &self.gen_func;
        hoge(&mut model, self.sequence.get());
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
