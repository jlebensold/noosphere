use anyhow::Result;
use async_trait::async_trait;
use cid::Cid;
use noosphere_storage::interface::Store;

use crate::slashlink::Slashlink;

#[derive(Clone)]
pub struct TextTransclude {
    pub title: String,
    pub author: String,
    pub excerpt: Option<String>,
    pub link_text: String,
}

#[derive(Clone)]
pub enum Transclude {
    // TODO
    // Rich,
    // Interactive,
    // Bitmap,
    Text(TextTransclude),
    None,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Transcluder {
    async fn make_transclude<S: Store>(
        &self,
        host_sphere: &Cid,
        host_content: &Cid,
        guest_sphere: &Cid,
        link: &Slashlink,
        block_store: &S,
    ) -> Result<Transclude>;
}
