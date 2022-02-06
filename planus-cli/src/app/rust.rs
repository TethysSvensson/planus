use std::path::PathBuf;

use anyhow::Result;

use crate::codegen::rust::generate_code;
use clap::{Parser, ValueHint};

/// Generate rust code
#[derive(Parser)]
pub struct Command {
    #[clap(value_hint = ValueHint::FilePath)]
    files: Vec<PathBuf>,

    /// Output file
    #[clap(short = 'o')]
    #[clap(value_hint = ValueHint::AnyPath)]
    output_filename: PathBuf,
}

impl Command {
    pub fn run(self, _options: super::AppOptions) -> Result<()> {
        generate_code(&self.files, &self.output_filename)?;

        Ok(())
    }
}
