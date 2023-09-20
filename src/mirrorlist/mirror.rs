use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, Serialize, Deserialize)]
pub struct Mirror {
    pub url: String,
    pub protocol: String,
    pub last_sync: Option<String>,
    pub completion_pct: f32,
    pub delay: Option<i32>,
    pub duration_avg: Option<f32>,
    pub duration_stddev: Option<f32>,
    pub score: Option<f64>,
    pub active: bool,
    pub country: String,
    pub country_code: String,
    pub isos: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: String,
}

impl fmt::Display for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "Server = {}$repo/os/$arch", self.url)
    }
}

impl fmt::Debug for Mirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| URL: {} | C: {} | P: {} |",
            self.url, self.country, self.protocol
        )
    }
}
