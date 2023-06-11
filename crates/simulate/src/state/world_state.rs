use dyn_clone::DynClone;


use crate::state::update::Update;

pub trait WorldState: DynClone + Sync + Send {
    type WorldStateUpdate: Update;
    fn execute(&mut self, update: &Self::WorldStateUpdate);
}

dyn_clone::clone_trait_object!(<U> WorldState<WorldStateUpdate = U> where U: Update);
