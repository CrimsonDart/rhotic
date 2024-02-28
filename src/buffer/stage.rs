use std::marker::PhantomData;



pub trait Stage where Self: Default {
    fn get_functions() -> &'static [(&'static str, fn(&mut Self) -> bool)];
    fn input_text(&mut self, string: &str);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function<S: Stage> {
    name: &'static str,
    func: fn(&mut S) -> bool,
    pd: PhantomData<S>
}

impl<S: Stage> Function<S> {
    fn new(name: &'static str, func: fn(&mut S) -> bool) -> Self {
        Self {
            name,
            func,
            pd: PhantomData
        }
    }
}

