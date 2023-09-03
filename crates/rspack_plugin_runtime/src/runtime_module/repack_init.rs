use rspack_core::{
  rspack_sources::{BoxSource, RawSource, SourceExt},
  ChunkUkey, Compilation, RuntimeModule,
};
use rspack_identifier::Identifier;

use crate::impl_runtime_module;

#[derive(Debug, Eq)]
pub struct RepackInitRuntimeModule {
  id: Identifier,
  chunk: ChunkUkey,
}

impl RepackInitRuntimeModule {
  pub fn new(chunk: &ChunkUkey) -> Self {
    Self {
      id: Identifier::from("repack-init"),
      chunk: *chunk,
    }
  }
}

impl RuntimeModule for RepackInitRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    let hmr_enabled: bool =
      compilation.options.mode.is_development() && compilation.options.dev_server.hot;

    RawSource::from(
      include_str!("runtime/repack_init.js")
        .replace("$hmrEnabled$", if hmr_enabled { "true" } else { "false" })
        .replace(
          "$chunkId$",
          &compilation
            .chunk_by_ukey
            .get(&self.chunk)
            .map_or("undefined", |chunk| {
              chunk.name.as_deref().unwrap_or("undefined")
            }),
        )
        .replace(
          "$chunkLoadingGlobal$",
          &compilation.options.output.chunk_loading_global,
        )
        .replace("$globalObject$", &compilation.options.output.global_object),
    )
    .boxed()
  }
}

impl_runtime_module!(RepackInitRuntimeModule);
