use std::{
    borrow::Cow,
    fs::File,
    io::{BufWriter, Write},
};

use auto_impl::auto_impl;

use crate::state::{update::Update, State, World};

#[auto_impl(Box)]
pub trait SummaryGenerator<U, W>
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    /// Yield the summary on the given world state.
    fn get_summary(&self, sim_state: &State<U, W>) -> Result<serde_json::Value, anyhow::Error>;

    /// The name of the summary.
    fn get_summary_name(&self) -> Cow<'static, str>;
}

pub struct Summarizer<U, W>
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    summary_generators: Vec<Box<dyn SummaryGenerator<U, W>>>,

    // Output File Name
    writer: BufWriter<File>,
}

impl<U, W> Summarizer<U, W>
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    pub fn new(file_name: Cow<'static, str>) -> Self {
        let file = File::create(file_name.as_ref()).expect("Unable to open output file.");
        Self {
            summary_generators: Vec::new(),
            writer: BufWriter::new(file),
        }
    }

    pub fn output_summaries(&mut self, sim_state: &State<U, W>) {
        for summary_generator in &self.summary_generators {
            let summary_result = summary_generator.get_summary(sim_state);
            log::debug!(
                "{}: {:?}",
                summary_generator.get_summary_name(),
                summary_result
            );
            if let Ok(val) = summary_result {
                writeln!(self.writer, "{}", val).expect("Unable to write results to output file.");
                self.writer
                    .flush()
                    .expect("Unable to write results to output file.");
            }
        }
    }

    pub fn register_summary_generator(&mut self, generator: Box<dyn SummaryGenerator<U, W>>) {
        self.summary_generators.push(generator);
    }

    pub fn get_num_summary_generators(&self) -> usize {
        self.summary_generators.len()
    }

    pub fn get_summary_generator(&self, index: usize) -> &dyn SummaryGenerator<U, W> {
        &self.summary_generators[index]
    }
}
