// From Blandy & Orendorff's *Programming Rust*

trait Vegetable {
    fn desc(&self) -> &'static str;
}

#[derive(Clone, Copy)]
struct Lettuce;
impl Vegetable for Lettuce {
    fn desc(&self) -> &'static str {
        "lettuce"
    }
}

#[derive(Clone, Copy)]
struct Tomato;
impl Vegetable for Tomato {
    fn desc(&self) -> &'static str {
        "tomato"
    }
}

struct GenericSalad<T: Vegetable> {
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

    fn display(&self) {
        for v in &self.veggies {
            println!("{}", v.desc());
        }
    }
}

struct TraitSalad<'a> {
    veggies: Vec<&'a dyn Vegetable>,
}

impl<'a> TraitSalad<'a> {
    fn new() -> Self {
        TraitSalad {
            veggies: Vec::new(),
        }
    }

    fn add(&mut self, v: &'a dyn Vegetable) {
        self.veggies.push(v);
    }

    fn display(&self) {
        for v in &self.veggies {
            println!("{}", v.desc());
        }
    }
}

fn main() {
    let mut generic_salad = GenericSalad::new();
    for v in vec![Lettuce, Lettuce] {
        generic_salad.add(v);
    }
    generic_salad.display();

    println!();

    let mut trait_salad = TraitSalad::new();
    trait_salad.add(&Lettuce);
    trait_salad.add(&Tomato);
    trait_salad.display();
}
