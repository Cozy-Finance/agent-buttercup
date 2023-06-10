use crate::state::update::Update;

pub trait WorldState: Sync + Send {
    type WorldStateUpdate: Update;
    fn execute(&mut self, update: Self::WorldStateUpdate);
}
