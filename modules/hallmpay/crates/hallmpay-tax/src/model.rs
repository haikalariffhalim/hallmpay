use common_domain::country::CountryCode;

#[derive(Debug, Clone)]
pub struct Address {
    pub country: Option<CountryCode>,
    pub postal_code: Option<String>,
    pub line1: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>, // ISO 3166-2
}

#[derive(Debug, Clone)]
pub struct IncomeTax {
    pub tax_code: String,
    pub name: String,
    pub rate: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct BusinessIncomeTax {
    pub ssm_number: Option<String>,
    pub ssm_number_format_valid: bool,
    pub income_tax: Vec<IncomeTax>,
    pub tax_exempt: bool,
    pub billing_address: Address,
}

#[derive(Debug)]
pub enum IncomeTax {
    incomeTaxRate(rust_decimal::Decimal),
    incomeTaxRates(Vec<IncomeTax>),
    ResolvedTaxRate(world_tax::TaxRate),
    ResolvedMultipleTaxRates(Vec<world_tax::TaxRate>),
    Exempt,
    NoTax,
}

impl Clone for IncomeTax {
    fn clone(&self) -> Self {
        match self {
            IncomeTax::incomeTaxRate(rate) => IncomeTax::incomeTaxRate(*rate),
            IncomeTax::incomeTaxRates(rates) => IncomeTax::incomeTaxRates(rates.clone()),
            IncomeTax::ResolvedTaxRate(tax_rate) => {
                IncomeTax::ResolvedTaxRate(world_tax::TaxRate {
                    rate: tax_rate.rate,
                    tax_type: tax_rate.tax_type.clone(),
                    compound: tax_rate.compound,
                })
            }
            IncomeTax::ResolvedMultipleTaxRates(rates) => IncomeTax::ResolvedMultipleTaxRates(
                rates
                    .iter()
                    .map(|tax_rate| world_tax::TaxRate {
                        rate: tax_rate.rate,
                        tax_type: tax_rate.tax_type.clone(),
                        compound: tax_rate.compound,
                    })
                    .collect(),
            ),
            IncomeTax::Exempt => IncomeTax::Exempt,
            IncomeTax::NoTax => IncomeTax::NoTax,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaxRule {
    pub country: Option<CountryCode>,
    pub region: Option<String>,
    pub rate: rust_decimal::Decimal,
}

pub struct TaxEntry {
    pub reference: String,
    pub name: String,
    pub rate: rust_decimal::Decimal,
    pub taxable_amount: u64,
    pub tax_amount: u64,
    pub is_exempt: bool,
}

pub struct TaxRateEntry {
    pub reference: String,
    pub name: String,
    pub rate: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct incomeTax {
    pub reference: String,
    pub name: String,
    pub tax_rules: Vec<TaxRule>,
}

pub struct LineItemForTax {
    pub line_id: String,
    pub amount: u64,
    pub income_taxes: Vec<incomeTax>,
}

#[derive(Debug, Clone)]
pub struct LineItemWithTax {
    pub line_id: String,
    pub pre_tax_amount: u64,
    pub tax_details: TaxDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatExemptionReason {
    TaxExempt,
    ReverseCharge,
    NotRegistered,
}

#[derive(Debug, Clone)]
pub struct TaxItem {
    pub tax_rate: rust_decimal::Decimal,
    pub tax_reference: String,
    pub tax_name: String,
    pub tax_amount: u64,
}

#[derive(Debug, Clone)]
pub enum TaxDetails {
    Tax {
        tax_rate: rust_decimal::Decimal,
        tax_reference: String,
        tax_name: String,
        tax_amount: u64,
    },
    MultipleTaxes {
        taxes: Vec<TaxItem>,
        total_tax_amount: u64,
    },
    Exempt(VatExemptionReason),
}

pub struct TaxBreakdownItem {
    pub taxable_amount: u64,
    pub details: TaxDetails,
}

pub struct CalculationResult {
    pub tax_amount: u64,
    pub total_amount_after_tax: u64,
    pub breakdown: Vec<TaxBreakdownItem>,
    pub line_items: Vec<LineItemWithTax>,
}

pub enum VatNumberExternalValidationResult {
    Valid,
    Invalid,
    ServiceUnavailable,
}
