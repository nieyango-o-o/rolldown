use std::path::{Component, PathBuf};

use rolldown_common::Output;
use rolldown_testing::case::{Case, Fixture};
use sugar_path::SugarPath;
use testing_macros::fixture;

#[allow(clippy::needless_pass_by_value)]
#[fixture("./tests/fixtures/**/_config.json")]
fn fixture_with_config(config_path: PathBuf) {
  Case::new(config_path.parent().unwrap()).run();
}

#[tokio::test(flavor = "multi_thread")]
async fn filename_with_hash() {
  let mut snapshot_outputs = vec![];

  let mut config_paths =
    glob::glob("./tests/**/_config.json").unwrap().map(Result::unwrap).collect::<Vec<_>>();
  let cwd = std::env::current_dir().unwrap();
  config_paths.sort_by_cached_key(|p| p.relative(&cwd));

  for path in config_paths {
    if path.components().map(Component::as_os_str).any(|c| c.to_string_lossy().starts_with('.')) {
      continue;
    }
    let mut snapshot_output = String::new();
    let config_path = path.canonicalize().unwrap();
    let config_path = dunce::simplified(&config_path);

    let mut fixture = Fixture::new(config_path.parent().unwrap().to_path_buf());
    snapshot_output
      .push_str(&format!("# {}\n\n", fixture.dir_path().relative(&cwd).to_slash_lossy()));

    let assets = fixture.bundle(false, true).await;

    assets.assets.iter().for_each(|asset| match asset {
      Output::Asset(asset) => {
        snapshot_output.push_str(&format!("- {}\n", asset.filename));
      }
      Output::Chunk(chunk) => {
        snapshot_output.push_str(&format!(
          "- {} => {}\n",
          chunk.preliminary_filename.as_str(),
          chunk.filename.as_str()
        ));
      }
    });

    snapshot_outputs.push(snapshot_output);
  }
  insta::assert_snapshot!(snapshot_outputs.join("\n"));
}
