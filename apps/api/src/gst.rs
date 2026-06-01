use crate::api::tax::Tax;
use crate::tax::gst::{StandardRate, ZeroRate};
use crate::types::TaxInfo;

pub trait Gst: Tax {
    /// Make object with GST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance after applying GST.
    fn after_gst(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        Self::after_tax(amount.into(), StandardRate::new())
    }

    /// Make object before applying GST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance before applying GST.
    fn before_gst(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.use_gst_standard_rate();
        instance
    }

    /// Make object without GST.
    ///
    /// # Arguments
    ///
    /// * `amount` - An integer or numeric string representing the amount.
    ///
    /// # Returns
    ///
    /// * `Self` - An instance without GST.
    fn without_gst(amount: impl Into<f64>) -> Self
    where
        Self: Sized,
    {
        let mut instance = Self::new(amount.into());
        instance.use_gst_zero_rate();
        instance
    }

    /// Enable GST for calculation.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The instance with GST enabled.
    fn use_gst_standard_rate(&mut self) -> &mut Self {
        self.enable_tax(StandardRate::new());
        self
    }

    /// Disable GST for calculation.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The instance with GST disabled.
    fn use_gst_zero_rate(&mut self) -> &mut Self {
        self.enable_tax(ZeroRate::new());
        self
    }
}
