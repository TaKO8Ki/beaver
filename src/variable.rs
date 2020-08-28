use phf::phf_map;

pub const ALPHABET: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

pub static ALPHABET_INDEX: phf::Map<&'static str, u8> = phf_map! {
    "a" => 0,
    "b" => 1,
    "c" => 2,
    "d" => 3,
    "e" => 4,
    "f" => 5,
    "g" => 6,
    "h" => 7,
    "i" => 8,
    "j" => 9,
    "k" => 10,
    "l" => 11,
    "m" => 12,
    "n" => 13,
    "o" => 14,
    "p" => 15,
    "q" => 16,
    "r" => 17,
    "s" => 18,
    "t" => 19,
    "u" => 20,
    "v" => 21,
    "w" => 22,
    "x" => 23,
    "y" => 24,
    "z" => 25,
};
