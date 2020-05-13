use super::*;

mod simple;

pub trait Material: Debug + DynClone {
    // todo
}

dyn_clone::clone_trait_object!(Material);
