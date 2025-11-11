use anyhow::{Result, anyhow};

use std::pin::Pin;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use crate::cache::CacheStore;
use crate::cipher::decipher::{SignatureDecipher, SignatureDecipherHandle};
use crate::yt_interface::{YtManifest, YtStreamResponse, YtVideoInfo};
use crate::{
    extractor::extract::{InfoExtractor, YtExtractor},
    yt_interface::VideoId,
};

pub struct Ty {
    yt_extractor: Arc<Mutex<YtExtractor>>,
    signature_decipher: Arc<Mutex<SignatureDecipher>>,
}

impl Ty {
    pub fn new() -> Result<Self> {
        let player_cache = Arc::new(CacheStore::new());
        let code_cache = Arc::new(CacheStore::new());

        let yt_extractor = YtExtractor::new(player_cache.clone(), code_cache.clone())?;
        let signature_decipher = SignatureDecipher::new(player_cache, code_cache);

        Ok(Self {
            yt_extractor: Arc::new(Mutex::new(yt_extractor)),
            signature_decipher: Arc::new(Mutex::new(signature_decipher)),
        })
    }
}

pub trait Extract {
    /// Extract the raw JSON manifest from YouTube's API.
    fn get_manifest<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractManifestFut<'a>;
    /// Extract metadata of a video from YouTube.
    fn get_video_info<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractInfoFut<'a>;
    /// Fetch and parse general video information (metadata) from an already fetched manifest.
    fn get_video_info_from_manifest<'a>(
        &'a self,
        manifest: &'a YtManifest,
    ) -> Self::ExtractInfoFut<'a>;
    /// Fetch and parse the streams from an already fetched manifest.
    fn get_streams_from_manifest<'a>(
        &'a self,
        manifest: &'a YtManifest,
    ) -> Self::ExtractStreamFut<'a>;
    type DecipherFut<'a>: Future<Output = Result<String>> + 'a
    where
        Self: 'a;
    /// Deciphers a stream's signature and returns it's URL.
    fn decipher_signature<'a>(
        &'a self,
        signature: String,
        player_url: String,
    ) -> Self::DecipherFut<'a>;
    /// Extract playable streams from YouTube and get their source either as a `Signature` or an `URL`
    fn get_streams<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractStreamFut<'a>;
    type ExtractStreamFut<'a>: Future<Output = Result<YtStreamResponse>> + 'a
    where
        Self: 'a;
    type ExtractInfoFut<'a>: Future<Output = Result<YtVideoInfo>> + 'a
    where
        Self: 'a;
    type ExtractManifestFut<'a>: Future<Output = Result<YtManifest>> + 'a
    where
        Self: 'a;
}

impl Extract for Ty {
    type ExtractStreamFut<'a> = Pin<Box<dyn Future<Output = Result<YtStreamResponse>> + 'a>>;
    type DecipherFut<'a> = Pin<Box<dyn Future<Output = Result<String>> + 'a>>;
    type ExtractInfoFut<'a> = Pin<Box<dyn Future<Output = Result<YtVideoInfo>> + 'a>>;
    type ExtractManifestFut<'a> = Pin<Box<dyn Future<Output = Result<YtManifest>> + 'a>>;

    fn get_streams<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractStreamFut<'a> {
        Box::pin(async move {
            let extractor = self
                .yt_extractor
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            extractor.extract_streams(video_id).await
        })
    }

    fn get_manifest<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractManifestFut<'a> {
        Box::pin(async move {
            let extractor = self
                .yt_extractor
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            extractor.extract_manifest(video_id).await
        })
    }

    fn get_video_info<'a>(&'a self, video_id: &'a VideoId) -> Self::ExtractInfoFut<'a> {
        Box::pin(async move {
            let extractor = self
                .yt_extractor
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            extractor.extract_video_info(video_id).await
        })
    }

    fn get_streams_from_manifest<'a>(
        &'a self,
        manifest: &'a YtManifest,
    ) -> Self::ExtractStreamFut<'a> {
        Box::pin(async move {
            let extractor = self
                .yt_extractor
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            extractor.extract_streams_from_manifest(manifest).await
        })
    }

    fn get_video_info_from_manifest<'a>(
        &'a self,
        manifest: &'a YtManifest,
    ) -> Self::ExtractInfoFut<'a> {
        Box::pin(async move {
            let extractor = self
                .yt_extractor
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            extractor.extract_video_info_from_manifest(manifest).await
        })
    }

    fn decipher_signature<'a>(
        &'a self,
        signature: String,
        player_url: String,
    ) -> Self::DecipherFut<'a> {
        Box::pin(async move {
            let signature_decipher = self
                .signature_decipher
                .lock()
                .map_err(|e| anyhow!(e.to_string()))?;
            signature_decipher.decipher(signature, player_url).await
        })
    }
}

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::{JsValue, prelude::*};
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen_futures::wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen(js_name = "fetchYtStreams")]
// pub async fn wasm_fetch_yt_streams(video_id: &str) -> JsValue {
//     let Ok(video_id_parsed) = VideoId::new(video_id) else {
//         panic!("Invalid Video ID.")
//     };

//     match Ty::extract(&video_id_parsed).await {
//         Ok(streams) => JsValue::from_str(""),
//         Err(err) => panic!("{}", err),
//     }
// }
