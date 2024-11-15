use bevy::prelude::{Resource, Transform};
use std::{path::PathBuf, time::Duration};

/// Type used to dictate various settings for this crate.
///
/// Download Settings:
/// - `endpoint` - Tile server endpoint (example: <https://tile.openstreetmap.org>)
/// - `tiles_directory` - The folder that all tiles will be stored in
/// - `max_concurrent_downloads` - Maximum number of concurrent tile downloads
/// - `max_retries` - Maximum number of retry attempts for failed downloads
/// - `rate_limit_requests` - Maximum number of requests allowed within the rate limit window
/// - `rate_limit_window` - Duration of the rate limit window
///
/// Display Settings:
/// - `reference_latitude` - Latitude that maps to Transform(0,0,0) or transform_offset if specified
/// - `reference_longitude` - Longitude that maps to Transform(0,0,0) or transform_offset if specified
/// - `transform_offset` - Optional offset from 0,0 where the reference coordinates should appear
/// - `z_layer` - Z coordinate for rendered tiles
/// - `auto_render` - Whether tiles should be automatically rendered
#[derive(Clone, Resource)]
pub struct SlippyTilesSettings {
    // Download settings
    pub endpoint: String,
    pub tiles_directory: PathBuf,
    pub max_concurrent_downloads: usize,
    pub max_retries: u32,
    pub rate_limit_requests: usize,
    pub rate_limit_window: Duration,

    // Display settings
    pub reference_latitude: f64,
    pub reference_longitude: f64,
    pub transform_offset: Option<Transform>,
    pub z_layer: f32,
    pub auto_render: bool,
}

impl SlippyTilesSettings {
    pub fn get_tiles_directory_string(&self) -> String {
        self.tiles_directory.as_path().to_str().unwrap().to_string()
    }
}

impl Default for SlippyTilesSettings {
    fn default() -> Self {
        Self {
            // Download defaults
            endpoint: "https://tile.openstreetmap.org".into(),
            tiles_directory: PathBuf::from("tiles/"),
            max_concurrent_downloads: 4,
            max_retries: 3,
            rate_limit_requests: 10,
            rate_limit_window: Duration::from_secs(1),

            // Display defaults
            reference_latitude: 0.0,
            reference_longitude: 0.0,
            transform_offset: None,
            z_layer: 0.0,
            auto_render: true,
        }
    }
}
