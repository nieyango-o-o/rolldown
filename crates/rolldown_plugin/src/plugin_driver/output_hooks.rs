use crate::types::hook_render_error::HookRenderErrorArgs;
use crate::PluginDriver;
use crate::{HookAugmentChunkHashReturn, HookNoopReturn, HookRenderChunkArgs};
use anyhow::Result;
use rolldown_common::{Output, RollupRenderedChunk};
use rolldown_sourcemap::SourceMap;

impl PluginDriver {
  pub async fn render_start(&self) -> HookNoopReturn {
    for (plugin, ctx) in &self.plugins {
      plugin.render_start(ctx).await?;
    }
    Ok(())
  }

  pub async fn render_chunk(
    &self,
    mut args: HookRenderChunkArgs<'_>,
  ) -> Result<(String, Vec<SourceMap>)> {
    let mut sourcemap_chain = vec![];
    for (plugin, ctx) in &self.plugins {
      if let Some(r) = plugin.render_chunk(ctx, &args).await? {
        args.code = r.code;
        if let Some(map) = r.map {
          sourcemap_chain.push(map);
        }
      }
    }
    Ok((args.code, sourcemap_chain))
  }

  pub async fn augment_chunk_hash(
    &self,
    chunk: &RollupRenderedChunk,
  ) -> HookAugmentChunkHashReturn {
    let mut hash = None;
    for (plugin, ctx) in &self.plugins {
      if let Some(plugin_hash) = plugin.augment_chunk_hash(ctx, chunk).await? {
        hash.get_or_insert_with(String::default).push_str(&plugin_hash);
      }
    }
    Ok(hash)
  }

  pub async fn render_error(&self, args: &HookRenderErrorArgs) -> HookNoopReturn {
    for (plugin, ctx) in &self.plugins {
      plugin.render_error(ctx, args).await?;
    }
    Ok(())
  }

  pub async fn generate_bundle(&self, bundle: &mut Vec<Output>, is_write: bool) -> HookNoopReturn {
    for (plugin, ctx) in &self.plugins {
      plugin.generate_bundle(ctx, bundle, is_write).await?;
      ctx.file_emitter.add_additional_files(bundle);
    }
    Ok(())
  }

  pub async fn write_bundle(&self, bundle: &mut Vec<Output>) -> HookNoopReturn {
    for (plugin, ctx) in &self.plugins {
      plugin.write_bundle(ctx, bundle).await?;
      ctx.file_emitter.add_additional_files(bundle);
    }
    Ok(())
  }
}
