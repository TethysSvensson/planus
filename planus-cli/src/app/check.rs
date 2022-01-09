use anyhow::Result;

use crate::{ast_map::AstMap, ctx::Ctx, intermediate_language::translation::Translator};
use clap::StructOpt;

/// Checks validity of files
#[derive(StructOpt)]
pub struct Command {
    files: Vec<String>,
}

impl Command {
    pub fn run(self, _options: super::AppOptions) -> Result<()> {
        let mut ctx = Ctx::default();
        let mut ast_map = AstMap::default();
        for file in self.files {
            if let Some(file_id) = ctx.add_file(&file, []) {
                ast_map.add_files_recursively(&mut ctx, file_id);
            }
        }

        let mut translator = Translator::new(&ctx, ast_map.reachability());
        for schema in ast_map.iter() {
            translator.add_schema(schema);
        }

        let _ = translator.finish();

        if ctx.has_errors() {
            anyhow::bail!("Bailing because of errors");
        }

        Ok(())
    }
}
