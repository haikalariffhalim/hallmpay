use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tax {
    pub id: String,
    pub tax_type: String,
    pub tax_rate: f64,
    #[serde(default)]
    pub tax: Option<String>,
}
