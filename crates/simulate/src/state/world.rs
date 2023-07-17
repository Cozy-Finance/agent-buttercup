use crate::state::update::UpdateData;

pub trait World: Sync + Send {
    type WorldUpdateData: UpdateData;
    fn execute(&mut self, update: Self::WorldUpdateData) -> Option<Self::WorldUpdateData>;
}
