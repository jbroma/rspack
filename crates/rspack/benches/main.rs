use criterion::{criterion_group, criterion_main, Criterion};

use std::path::PathBuf;

use node_binding::{normalize_bundle_options, RawOptions};
use temp_test_utils::RawOptionsTestExt;

async fn bench(cur_dir: &PathBuf) {
  // cur_dir = cur_dir.join("webpack_css_cases_to_be_migrated/bootstrap");
  let options = normalize_bundle_options(RawOptions::from_fixture(cur_dir))
    .unwrap_or_else(|_| panic!("failed to normalize in fixtrue {:?}", cur_dir));
  println!("{:?}", options);
  let mut compiler = rspack::rspack(options, Default::default());

  let _stats = compiler
    .run()
    .await
    .unwrap_or_else(|_| panic!("failed to compile in fixtrue {:?}", cur_dir));
}

fn criterion_benchmark(c: &mut Criterion) {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();
  let mut cur_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR").unwrap());
  cur_dir = cur_dir.join("../../examples/bench");
  cur_dir = cur_dir.canonicalize().unwrap();
  c.bench_function("rspack", |b| b.to_async(&rt).iter(|| bench(&cur_dir)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);