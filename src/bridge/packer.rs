use pyo3::prelude::*;
use crate::bridge::BridgeConfig;

#[pyclass]
pub struct SovereignPacker;

#[pymethods]
impl SovereignPacker {
    #[new]
    pub fn new() -> Self {
        Self
    }

    #[static_method]
    pub fn bundle_sovereign_environment(self_contained: bool) -> PyResult<()> {
        if !self_contained {
            return Ok(());
        }

        Python::with_gil(|py| {
            let sys = py.import_bound("sys")?;
            sys.getattr("path")?.call_method1("append", (".",))?;
            Ok(())
        })
    }

    #[static_method]
    pub fn freeze_logic(script_source: &str) -> Vec<u8> {
        script_source.as_bytes().to_vec() 
    }
}

impl SovereignPacker {
    pub fn bundle_internal(config: &BridgeConfig) -> PyResult<()> {
        Self::bundle_sovereign_environment(config.self_contained)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bridge::BridgeConfig;

    #[test]
    fn test_environment_initialization() {
        let config = BridgeConfig::default();
        let result = SovereignPacker::bundle_internal(&config);
        if cfg!(feature = "extension-module") {
            assert!(result.is_ok());
        }
    }
}
