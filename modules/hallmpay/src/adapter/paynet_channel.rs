
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaynetChannelCode {
	#[serde(with = "string_serde")]
	pub client_id: String,

}

Channel can be any of the following:
RB – Retail Internet Banking
CB – Corporate Internet Banking
RM – Retail Mobile Banking
CM – Corporate Mobile Banking
OT - Other
BW – Business Web (To be used for FI initiated Merchant Transactions or Merchant Initiated Transactions)
BA - Business App (To be used for FI initiated Merchant Transactions or Merchant Initiated Transactions)
QR - QR Payment (For DuitNow QR use only)
