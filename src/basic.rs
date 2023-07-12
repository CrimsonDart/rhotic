


//! The name must be unique! The name is used as a unique identifier for a widget, so dont screw it up!
pub trait Name {
    fn name(&self) -> &'static str;
}

pub enum Or<T, U> {
    First(T),
    Second(U)
}

impl<T, U> Or<T, U> {

    pub fn is_first(&self) -> bool {
        use Or::*;


        if let First(_) = self {
            true
        } else {
            false
        }
    }

}
