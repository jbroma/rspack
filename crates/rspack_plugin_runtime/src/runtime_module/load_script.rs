use rspack_core::{
  rspack_sources::{BoxSource, RawSource, SourceExt},
  ChunkUkey, Compilation, RuntimeGlobals, RuntimeModule,
};
use rspack_identifier::Identifier;

use crate::impl_runtime_module;

#[derive(Debug, Eq)]
pub struct LoadScriptRuntimeModule {
  id: Identifier,
  chunk: ChunkUkey,
}

impl LoadScriptRuntimeModule {
  pub fn new(chunk: &ChunkUkey) -> Self {
    Self {
      id: Identifier::from("webpack/runtime/load_script"),
      chunk: *chunk,
    }
  }
}

impl RuntimeModule for LoadScriptRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    RawSource::from(
      include_str!("runtime/load_script.js")
        .replace("$loadScript$", RuntimeGlobals::LOAD_SCRIPT.name())
        .replace(
          "$caller$",
          &compilation
            .chunk_by_ukey
            .get(&self.chunk)
            .map_or("undefined", |chunk| {
              chunk.name.as_deref().unwrap_or("undefined")
            }),
        ),
    )
    .boxed()
  }
}

impl_runtime_module!(LoadScriptRuntimeModule);
