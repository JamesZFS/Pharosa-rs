use super::*;

mod simple;

pub use simple::Simple;

pub trait BSDF: DynClone + Debug {
    //todo
}

dyn_clone::clone_trait_object!(BSDF);
