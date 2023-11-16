use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

impl std::fmt::Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Server = {}$repo/os/$arch", self.url)
    }
}
