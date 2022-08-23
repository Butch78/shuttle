pub mod persist_derive;

pub trait Persist {
    fn save(&self);

    fn load(&self) -> Self;
}
