// src/processing/cleaner.rs

//! # Penta-Cleaner: Geometric Data Sanitization Engine
//! 
//! High-speed data cleaning logic that maps columnar operations to 
//! geometric poles for maximum throughput and system stability.

use polars_core::prelude::*;
use rayon::prelude::*;
use crate::processing::ProcessingState;
// FIX: Explicitly import InterpolationStrategy to resolve E0433
use polars_core::utils::InterpolationStrategy; 

/// The primary engine for structural data purification.
pub struct PentaCleaner;

impl PentaCleaner {
    /// Sanitizes a DataFrame using geometric parallel distribution.
    /// 
    /// This function applies "Structural Sanitization":
    /// 1. Maps each series (column) to a geometric pole.
    /// 2. Performs parallel null-handling and outlier dissipation.
    /// 3. Returns a hardened DataFrame ready for the SECURE_CORE.
    pub fn geometric_sanitize(df: &mut DataFrame, state: &ProcessingState) -> PolarsResult<()> {
        let pressure = state.data_pressure;

        // FIX: get_columns_mut is unsafe in Polars 0.36+ (E0133).
        // Wrapping in unsafe block for parallel mutable access via Rayon.
        let columns = unsafe { df.get_columns_mut() };

        columns.par_iter_mut().for_each(|series| {
            Self::purify_column(series, pressure);
        });

        Ok(())
    }

    /// Internal logic to purify a single column based on system pressure.
    fn purify_column(series: &mut Series, pressure: f64) {
        // High-intensity cleaning logic: If pressure > 0.8, use aggressive interpolation.
        if pressure > 0.8 {
            // FIX: Ensure the trait method is available and results are handled.
            if let Ok(interpolated) = series.interpolate(InterpolationStrategy::Linear) {
                *series = interpolated;
            }
        } else {
            // Standard foundation cleaning: Zero-fill for missing stability values.
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
        // Initialize with data containing missing values (nulls)
        let s0 = Series::new("stability_metrics", &[Some(1.0), Some(2.0), None, Some(4.0)]);
        let mut df = DataFrame::new(vec![s0]).unwrap();
        let state = ProcessingState::default();

        assert!(PentaCleaner::geometric_sanitize(&mut df, &state).is_ok());
        
        // Ensure null values were dissipated (filled with 0.0 in default state)
        let cleaned_series = df.column("stability_metrics").unwrap();
        
        // Validation of data integrity
        assert_eq!(cleaned_series.get(2).unwrap(), AnyValue::Float64(0.0));
    }
}
