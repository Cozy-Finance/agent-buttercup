use crate::state::update::Update;

pub trait World: Sync + Send {
    type WorldUpdate: Update;
    fn execute(&mut self, update: Self::WorldUpdate) -> Option<Self::WorldUpdate>;
}
