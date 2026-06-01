# HallmPay

HallmPay- Malaysian Payment Gateway 

[![Crates.io](https://img.shields.io/crates/v/hallmpay)](https://crates.io/crates/hallmpay)
[![Docs.rs](https://docs.rs/hallmpay/badge.svg)](https://docs.rs/hallmpay)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

<img src="./public/home.png" alt="home" />
<img src="./public/fpx.png" alt="fpx" />

<img src="./public/payment.png" alt="payment" />


## Overall structure

hallmpay/

├── apps
│   ├── api/src
│   ├── engine/src
│  
│
├── lib/src
│   ├── hallmpay-core/
│   ├── hallmpay-paynet/
│   ├── hallmpay-fpx/
│   ├── hallmpay-duitnow/
│   ├── hallmpay-jompay/
│   ├── hallmpay-db/
│   ├── hallmpay-auth/
│   ├── hallmpay-events/
│   └── hallmpay-openapi/
│
└── convex/
    ├── docker/
    ├── migrations/
    └── scripts/
    
<img src="./public/docs.png" alt="docs" />

<img src="./public/changelog.png" alt="changelog" />
    
## Todo 
    
[ ]	 For HallmPay, I would start with:
[ ]  Core Axum API (Client)
[ ]  PayNet authentication module
[ ]  FPX integration
[ ]  DuitNow Online Banking/Wallets
[ ]  Webhook handling
[ ]  Reconciliation worker
[ ]  JomPAY
[ ]  DuitNow QR
[ ]  Merchant onboarding
[ ]  Multi-tenant merchant accounts
