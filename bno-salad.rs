#![allow(unused)]

// From Blandy & Orendorff's *Programming Rust*

pub trait Vegetable {
    fn desc(&self) -> &'static str;
}

#[derive(Clone, Copy)]
pub struct Lettuce;
impl Vegetable for Lettuce {
    fn desc(&self) -> &'static str {
        self.lettuce_desc()
    }
}

impl Lettuce {
    fn lettuce_desc(&self) -> &'static str {
        "lettuce"
    }
}

#[derive(Clone, Copy)]
pub struct Tomato;
impl Vegetable for Tomato {
    fn desc(&self) -> &'static str {
        self.tomato_desc()
    }
}

impl Tomato {
    fn tomato_desc(&self) -> &'static str {
        "tomato"
    }
}

#[derive(Clone, Copy)]
pub struct Cucumber;

impl Vegetable for Cucumber {
    fn desc(&self) -> &'static str {
        self.cucumber_desc()
    }
}

impl Cucumber {
    fn cucumber_desc(&self) -> &'static str {
        "cucumber"
    }
}

pub struct GenericSalad<T: Vegetable> {
    bowl: Vec<T>,
}

impl<T: Vegetable> GenericSalad<T> {
    fn new() -> Self {
        GenericSalad {
            bowl: Vec::new(),
        }
    }

    fn add(&mut self, v: T) {
        self.bowl.push(v);
    }

    pub fn display(&self) {
        for v in &self.bowl {
            println!("{}", v.desc());
        }
    }
}

pub struct TraitSalad<'a> {
    bowl: Vec<Box<dyn Vegetable + 'a>>,
}

impl<'a> TraitSalad<'a> {
    fn new() -> Self {
        TraitSalad {
            bowl: Vec::new(),
        }
    }

    fn add(&mut self, v: Box<dyn Vegetable>) {
        self.bowl.push(v);
    }

    fn add_direct<T: Vegetable + 'a>(&mut self, v: T) {
        self.bowl.push(Box::new(v));
    }

    pub fn display(&self) {
        for v in &self.bowl {
            println!("{}", v.desc());
        }
    }
}

enum SaladVegetable {
    SaladLettuce,
    SaladTomato,
    SaladCucumber,
}
use SaladVegetable::*;

pub struct EnumSalad {
    bowl: Vec<SaladVegetable>,
}

impl EnumSalad {
    fn new() -> Self {
        EnumSalad {
            bowl: Vec::new(),
        }
    }

    fn add(&mut self, v: SaladVegetable) {
        self.bowl.push(v);
    }

    pub fn display(&self) {
        for v in &self.bowl {
            match v {
                SaladLettuce => println!("lettuce"),
                SaladTomato => println!("tomato"),
                SaladCucumber => println!("cucumber"),
            }
        }
    }
}

pub fn make_me_a_generic_salad<T: Vegetable + Clone>(veggie: T) ->
    GenericSalad<T>
{
    let mut generic_salad = GenericSalad::new();
    for v in vec![veggie.clone(), veggie.clone()] {
        generic_salad.add(v);
    }
    generic_salad
}

pub fn make_me_a_trait_salad() -> TraitSalad<'static> {
    let mut trait_salad = TraitSalad::new();
    trait_salad.add_direct(Lettuce);
    trait_salad.add_direct(Tomato);
    trait_salad
}

pub fn make_me_an_enum_salad() -> EnumSalad {
    let mut enum_salad = EnumSalad::new();
    enum_salad.add(SaladLettuce);
    enum_salad.add(SaladTomato);
    enum_salad.add(SaladTomato);
    enum_salad.add(SaladCucumber);
    enum_salad
}

fn main() {
    let mut salad: GenericSalad<Lettuce> = make_me_a_generic_salad(Lettuce);
    salad.display();
    // salad.add(Tomato);
    // salad.display();
    
    println!();

    let salad = make_me_an_enum_salad();
    salad.display();

    println!();

    let mut salad = make_me_a_trait_salad();
    salad.display();
    salad.add_direct(Cucumber);
    salad.display();
}
