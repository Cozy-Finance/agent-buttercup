use eyre::Result;
use std::fs::File;
use std::io::Write;
use std::{borrow::Cow, io::BufWriter};

use crate::state::{update::UpdateData, world::World, SimState};

pub trait SummaryGenerator<U: UpdateData, W: World<WorldUpdateData = U>> {
    /// Yield the summary on the given world state.
    fn get_summary(&self, sim_state: &SimState<U, W>) -> Result<serde_json::Value>;

    /// The name of the summary.
    fn get_summary_name(&self) -> Cow<'static, str>;
}

/// Allows the Box<dyn SummaryGenerator> pointers to delegate the call to
/// get_summary to the heap stored object.
impl<U: UpdateData, W: World<WorldUpdateData = U>, S: SummaryGenerator<U, W>> SummaryGenerator<U, W>
    for Box<S>
{
    fn get_summary(&self, sim_state: &SimState<U, W>) -> Result<serde_json::Value> {
        (**self).get_summary(sim_state)
    }

    fn get_summary_name(&self) -> Cow<'static, str> {
        (**self).get_summary_name()
    }
}

pub struct Summarizer<U: UpdateData, W: World<WorldUpdateData = U>> {
    summary_generators: Vec<Box<dyn SummaryGenerator<U, W>>>,

    // Output File Name
    writer: BufWriter<File>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> Summarizer<U, W> {
    pub fn new(file_name: Cow<'static, str>) -> Self {
        let file = File::create(file_name.as_ref()).expect("Unable to open output file");
        Self {
            summary_generators: Vec::new(),
            writer: BufWriter::new(file),
        }
    }

    pub fn output_summaries(&mut self, sim_state: &SimState<U, W>) -> Result<()> {
        for summary_generator in &self.summary_generators {
            let summary_result = summary_generator.get_summary(sim_state);
            log::debug!(
                "{}: {:?}",
                summary_generator.get_summary_name(),
                summary_result
            );
            if let Ok(val) = summary_result {
                writeln!(self.writer, "{}", val)?;
                self.writer.flush().expect("Error writing results to file.");
            }
        }
        Ok(())
    }

    pub fn register_summary_generator(&mut self, generator: Box<dyn SummaryGenerator<U, W>>) {
        self.summary_generators.push(generator);
    }

    pub fn get_num_summary_generators(&self) -> usize {
        return self.summary_generators.len();
    }

    pub fn get_summary_generator(&self, index: usize) -> &Box<dyn SummaryGenerator<U, W>> {
        return &self.summary_generators[index];
    }
}
