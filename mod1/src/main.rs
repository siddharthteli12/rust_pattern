use once_cell::sync::OnceCell;

trait Run {
    fn run(&self);
}

struct Human;

impl Human {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Human> = OnceCell::new();
        INSTANCE.get_or_init(|| Human)
    }
}

impl Run for Human {
    fn run(&self) {
        println!("HUMAN IS RUNNING");
    }
}

struct Alien;

impl Alien {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Alien> = OnceCell::new();
        INSTANCE.get_or_init(|| Alien)
    }
}

impl Run for Alien {
    fn run(&self) {
        println!("ALIEN IS RUNNING");
    }
}

struct Dog;

impl Dog {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<Dog> = OnceCell::new();
        INSTANCE.get_or_init(|| Dog)
    }
}

impl Run for Dog {
    fn run(&self) {
        println!("DOG IS RUNNING");
    }
}

use std::env;

fn main() {
    dotenv::dotenv().ok();
    let value = env::var("INJECT").unwrap();

    let runnable = decide_dependency(&value);

    runnable.run();
}

// Definetly, slower than static binding. But still binding will happen at startup time. 
// Assuming when its being used in large backend.
fn decide_dependency(arg: &str) -> &'static dyn Run {
    match arg {
        "Dog" => Dog::global(),
        "Human" => Human::global(),
        "Alien" => Alien::global(),
        _ => unreachable!(),
    }
}
