use dyn_clone::DynClone;

use crate::state::update::UpdateData;

pub trait World: DynClone + Sync + Send {
    type WorldUpdateData: UpdateData;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData>;
}

dyn_clone::clone_trait_object!(<U> World<WorldUpdateData = U> where U: UpdateData);
