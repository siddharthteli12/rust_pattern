use once_cell::sync::OnceCell;

trait Run {
    fn run(&self);
}

macro_rules! singleton {
    ($type:tt) => {{
        static INSTANCE: OnceCell<$type> = OnceCell::new();
        INSTANCE.get_or_init(|| <$type>::default())
    }};
}

#[derive(Default)]
struct Human;

impl Run for Human {
    fn run(&self) {
        println!("HUMAN IS RUNNING");
    }
}

#[derive(Default)]
struct Alien;

impl Run for Alien {
    fn run(&self) {
        println!("ALIEN IS RUNNING");
    }
}

#[derive(Default)]
struct Dog;

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
        "Dog" => singleton!(Dog),
        "Human" => singleton!(Human),
        "Alien" => singleton!(Alien),
        _ => unreachable!(),
    }
}
