use crate::types::transaction;
use crate::types::Tax;

pub trait Tax: Calculation {
    /// Enable Tax calculation.
    ///
    /// Option<Box<dyn Taxable>> to represent nullable Taxable trait object
    fn taxable(&self) -> Option<&dyn Taxable>;

    /// Make object with Tax.
    fn after_tax(amount: impl Into<String>, taxable: Box<dyn Taxable>) -> Self
    where
        Self: Sized,
    {
        Self::before_tax(
            taxable.get_amount_without_tax(&Self::total_amount(amount.into())),
            taxable,
        )
    }

    /// Make object before applying Tax.
    fn before_tax(amount: impl Into<String>, taxable: Box<dyn Taxable>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.enable_tax(taxable);
        instance
    }

    /// Make object without Tax.
    fn without_tax(amount: impl Into<String>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.disable_tax();
        instance
    }

    /// Get formatted amount with GST.
    fn amount_with_tax(&self) -> String {
        self.get_formatter()
            .format(&Self::total_amount(self.get_amount_with_tax()))
    }

    /// Get formatted cash with GST.
    fn cash_amount_with_tax(&self) -> String {
        self.get_formatter()
            .format(&Self::as_money(self.get_cash_amount_with_tax()))
    }

    /// Enable Tax for calculation.
    fn enable_tax(&mut self, taxable: Box<dyn Taxable>) -> &mut Self;

    /// Disable Tax for calculation.
    fn disable_tax(&mut self) -> &mut Self;

    /// Check if the object has Tax.
    fn has_tax(&self) -> bool {
        self.taxable().is_some()
    }

    /// Get GST amount.
    fn get_tax_amount(&self) -> String {
        match self.taxable() {
            Some(taxable) => taxable.get_tax_amount(&self.get_money()),
            None => "0".to_string(),
        }
    }

    /// Returns the value represented by this object with Tax.
    fn get_amount_with_tax(&self) -> String {
        match self.taxable() {
            Some(taxable) => taxable.get_amount_with_tax(&self.get_money()),
            None => self.get_money().get_amount(),
        }
    }

    /// Get amount for cash with Tax.
    fn get_cash_amount_with_tax(&self) -> String {
        self.get_closest_accepted_cash_amount(&self.get_amount_with_tax())
            .to_string()
    }

    /// Allocate the money according to a list of ratios with Tax.
    fn allocate_with_tax(&self, ratios: &[f64]) -> Vec<Self>
    where
        Self: Sized,
    {
        let method = if self.has_tax() {
            "after_tax"
        } else {
            "without_tax"
        };

        let mut results = Vec::new();
        let allocates = Self::as_money(self.get_amount_with_tax()).allocate(ratios);

        for allocate in allocates {
            let amount = allocate.get_amount();
            let taxable = self.taxable();
            let instance = match method {
                "after_tax" => Self::after_tax(amount, taxable.unwrap().clone_box()),
                "without_tax" => Self::without_tax(amount),
                _ => unreachable!(),
            };
            results.push(instance);
        }

        results
    }

    /// Allocate the money among N targets with GST.
    fn allocate_with_tax_to(&self, n: usize) -> Vec<Self>
    where
        Self: Sized,
    {
        // Implementation depends on the rest of the trait and context
        unimplemented!()
    }
}
