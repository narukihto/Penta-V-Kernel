// src/processing/cleaner.rs

//! # Penta-Cleaner: Geometric Data Sanitization Engine
//! 
//! High-speed data cleaning logic that maps columnar operations to 
//! geometric poles for maximum throughput and system stability.

use polars_core::prelude::*;
use rayon::prelude::*;
use crate::processing::ProcessingState;

/// The primary engine for structural data purification.
pub struct PentaCleaner;

impl PentaCleaner {
    /// Sanitizes a DataFrame using geometric parallel distribution.
    pub fn geometric_sanitize(df: &mut DataFrame, state: &ProcessingState) -> PolarsResult<()> {
        let pressure = state.data_pressure;

        // Secure mutable access for parallel processing
        let columns = unsafe { df.get_columns_mut() };

        columns.par_iter_mut().for_each(|series| {
            Self::purify_column(series, pressure);
        });

        Ok(())
    }

    /// Internal logic to purify a single column based on system pressure.
    fn purify_column(series: &mut Series, pressure: f64) {
        // High-intensity cleaning: Use Mean filling for high pressure
        if pressure > 0.8 {
            // Forward fill handles the geometry sequence better than zero-fill
            if let Ok(filled) = series.fill_null(FillNullStrategy::Forward(None)) {
                *series = filled;
            }
        } else {
            // Standard foundation cleaning: Zero-fill for stable baseline
            if let Ok(filled) = series.fill_null(FillNullStrategy::Zero) {
                *series = filled;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processing::ProcessingState;

    #[test]
    fn test_geometric_purification() {
        let s0 = Series::new("stability_metrics", &[Some(1.0), Some(2.0), None, Some(4.0)]);
        let mut df = DataFrame::new(vec![s0]).unwrap();
        let state = ProcessingState::default();

        assert!(PentaCleaner::geometric_sanitize(&mut df, &state).is_ok());
        
        let cleaned_series = df.column("stability_metrics").unwrap();
        assert_eq!(cleaned_series.get(2).unwrap(), AnyValue::Float64(0.0));
    }
}
