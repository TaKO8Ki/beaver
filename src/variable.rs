use lazy_static::lazy_static;
use std::collections::HashMap;

pub const ALPHABET: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

lazy_static! {
    pub static ref ALPHABET_INDEX: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("a", 0);
        m.insert("b", 1);
        m.insert("c", 2);
        m.insert("d", 3);
        m.insert("e", 4);
        m.insert("f", 5);
        m.insert("g", 6);
        m.insert("h", 7);
        m.insert("i", 8);
        m.insert("j", 9);
        m.insert("k", 10);
        m.insert("l", 11);
        m.insert("m", 12);
        m.insert("n", 13);
        m.insert("o", 14);
        m.insert("p", 15);
        m.insert("q", 16);
        m.insert("r", 17);
        m.insert("s", 18);
        m.insert("t", 19);
        m.insert("u", 20);
        m.insert("v", 21);
        m.insert("w", 22);
        m.insert("x", 23);
        m.insert("y", 24);
        m.insert("z", 25);
        m
    };
}
