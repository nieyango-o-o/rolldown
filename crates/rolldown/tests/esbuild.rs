use std::path::PathBuf;

use rolldown_testing::case::Case;
use testing_macros::fixture;

#[allow(clippy::needless_pass_by_value)]
#[fixture("./tests/esbuild/**/_config.json")]
fn test(path: PathBuf) {
  Case::new(path.parent().unwrap()).run();
}
