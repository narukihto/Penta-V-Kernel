//! # Penta-V Build Script
//! 
//! Orchestrates the linking process for the PyO3 extension module.
//! Essential for preventing 'undefined symbol' errors in Unix-based CI environments.

fn main() {
    // Instruct the compiler to use PyO3 build configurations.
    // This resolves missing symbols like PyObject_Str during the linking phase.
    pyo3_build_config::add_extension_module_link_args();
}
