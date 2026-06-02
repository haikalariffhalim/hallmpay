use crate::api::tax::Tax;
use crate::model::TaxInfo;
use crate::tax::SST::{StandardRate, ZeroRate};

pub trait SST: Tax {
    /// Make object with SST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance after applying SST.
    fn after_SST(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        Self::after_tax(amount.into(), StandardRate::new())
    }

    /// Make object before applying SST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance before applying SST.
    fn before_SST(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.use_SST_standard_rate();
        instance
    }

    /// Make object without SST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance without SST.
    fn without_SST(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.use_SST_zero_rate();
        instance
    }

    /// Enable SST for calculation.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The instance with SST enabled.
    fn use_SST_standard_rate(&mut self) -> &mut Self {
        self.enable_tax(StandardRate::new());
        self
    }

    /// Disable SST for calculation.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The instance with SST disabled.
    fn use_SST_zero_rate(&mut self) -> &mut Self {
        self.enable_tax(ZeroRate::new());
        self
    }
}
