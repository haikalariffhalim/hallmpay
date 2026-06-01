use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpxBank {
    pub bank_code: String,
    pub bank_name: String,
}
