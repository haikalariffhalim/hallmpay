use crate::types::ApiVersion;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HallmPayConfig {
    pub token: String,
    pub sandbox: bool,
    pub api_version: ApiVersion,
    pub timeout: Duration,
}

impl HallmPayConfig {
    pub fn base_url(&self) -> &str {
        match (self.api_version, self.sandbox) {
            (ApiVersion::V2, true) => "https://api.console.hallmpay/sandbox",
            (ApiVersion::V1, false) => "https://api.console.hallmpay/cash",
            (ApiVersion::V2, false) => "https://console.hallmpay/api/sandbox",
            (ApiVersion::V1, true) => "https://console.hallmpay/api/cash",
        }
    }

    pub fn manual_transfer_base_url(&self) -> &str {
        if self.sandbox {
            "https://console.hallmpay/api/sandbox"
        } else {
            "https://console.hallmpay/api/cash"
        }
    }
}
