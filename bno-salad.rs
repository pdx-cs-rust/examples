// From Blandy & Orendorff's *Programming Rust*

pub trait Vegetable {
    fn desc(&self) -> &'static str;
}

#[derive(Clone, Copy)]
pub struct Lettuce;
impl Vegetable for Lettuce {
    fn desc(&self) -> &'static str {
        self.desc()
    }
}

#[derive(Clone, Copy)]
pub struct Tomato;
impl Vegetable for Tomato {
    fn desc(&self) -> &'static str {
        self.desc()
    }
}

pub struct GenericSalad<T: Vegetable> {
    veggies: Vec<T>,
}

impl<T: Vegetable> GenericSalad<T> {
    fn new() -> Self {
        GenericSalad {
            veggies: Vec::new(),
        }
    }

    fn add(&mut self, v: T) {
        self.veggies.push(v);
    }

    pub fn display(&self) {
        for v in &self.veggies {
            println!("{}", v.desc());
        }
    }
}

pub struct TraitSalad {
    veggies: Vec<Box<dyn Vegetable>>,
}

impl TraitSalad {
    fn new() -> Self {
        TraitSalad {
            veggies: Vec::new(),
        }
    }

    fn add(&mut self, v: Box<dyn Vegetable>) {
        self.veggies.push(v);
    }

    pub fn display(&self) {
        for v in &self.veggies {
            println!("{}", v.desc());
        }
    }
}

impl Lettuce {
    fn desc(&self) -> &'static str {
        "lettuce"
    }
}

impl Tomato {
    fn desc(&self) -> &'static str {
        "tomato"
    }
}

enum SaladVegetable {
    SaladLettuce,
    SaladTomato,
    SaladCucumber,
}
use SaladVegetable::*;

pub struct EnumSalad {
    veggies: Vec<SaladVegetable>,
}

impl EnumSalad {
    fn new() -> Self {
        EnumSalad {
            veggies: Vec::new(),
        }
    }

    fn add(&mut self, v: SaladVegetable) {
        self.veggies.push(v);
    }

    pub fn display(&self) {
        for v in &self.veggies {
            match v {
                SaladLettuce => println!("lettuce"),
                SaladTomato => println!("tomato"),
                SaladCucumber => println!("cucumber"),
            }
        }
    }
}

pub fn make_me_a_generic_lettuce_salad<T: Vegetable + Clone>(veggie: T) ->
    GenericSalad<T>
{
    let mut generic_salad = GenericSalad::new();
    for v in vec![veggie.clone(), veggie.clone()] {
        generic_salad.add(v);
    }
    generic_salad
}

pub fn make_me_a_trait_salad() -> TraitSalad {
    let mut trait_salad = TraitSalad::new();
    trait_salad.add(Box::new(Lettuce));
    trait_salad.add(Box::new(Tomato));
    trait_salad
}

pub fn make_me_a_enum_salad() -> EnumSalad {
    let mut enum_salad = EnumSalad::new();
    enum_salad.add(SaladLettuce);
    enum_salad.add(SaladTomato);
    enum_salad.add(SaladTomato);
    enum_salad.add(SaladCucumber);
    enum_salad
}
