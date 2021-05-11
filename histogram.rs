// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
struct Histogram<T: Hash + Eq>(HashMap<T, usize>);

impl<T: Hash + Eq> Histogram<T> {
    fn histogram<I>(values: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut h = HashMap::new();
        for k in values {
            h.entry(k).and_modify(|e| *e += 1).or_insert(1);
        }
        Histogram(h)
    }
}

impl<T: Hash + Eq + Ord> Histogram<T> {
    fn graph<'a>(&'a self) -> Vec<(&'a T, usize)> {
        let Histogram(h) = self;
        let mut result: Vec<(&T, usize)> =
            h.iter().map(|(k, v)| (k, *v)).collect();
        result.sort();
        result
    }
}

impl<T: Hash + Eq> FromIterator<T> for Histogram<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Histogram::histogram(iter)
    }
}

impl<T> fmt::Display for Histogram<T>
where
    T: Hash + Eq + Ord + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (key, count) in self.graph() {
            writeln!(fmt, "{:?}: {}", key, count)?;
        }
        Ok(())
    }
}

fn clean_chars(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_alphabetic() || c.is_numeric() || c.is_whitespace() {
            result.extend(c.to_lowercase());
        } else {
            result.push(' ');
        }
    }
    result
}

fn main() {
    let h: Histogram<char> = "hello world"
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("{}", h);

    let words: Histogram<&str> = "a banana is a banana is a banana"
        .split_whitespace()
        .collect();
    println!("{}", words);

    let mut gba = String::new();
    File::open("gettysburg-address.txt")
        .expect("file not found")
        .read_to_string(&mut gba)
        .expect("file read error");
    let gba = clean_chars(&gba);
    let gba: Histogram<&str> = gba.split_whitespace().collect();
    println!("{}", gba);
}
