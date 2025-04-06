use pyo3::prelude::*;
use pyo3::ffi::c_str;
use eyre::Result;
use std::fs;

pub struct LogAnalyzer {
    log_path: String,
}

impl LogAnalyzer {
    pub fn new(log_path: &str) -> Self {
        Self { log_path: log_path.to_string() }
    }

    pub fn analyze(&self) -> Result<()> {
        let contents = fs::read_to_string(&self.log_path)?;
        let lines: Vec<&str> = contents.lines().collect();

        Python::with_gil(|py| -> PyResult<()> {
            let analysis_module = PyModule::from_code(
                py,
                c_str!(include_str!("../analysis.py")),
                c_str!("analysis.py"), 
                c_str!("analysis"),
            )?;

            let result = analysis_module
                .getattr("analyze")?
                .call1((lines,))?;
            
            let top_domains: Vec<(String, usize)> = result.get_item("top_domains")?.extract()?;
            let top_ips: Vec<(String, usize)> = result.get_item("top_ips")?.extract()?;

            println!("Top Blocked Domains:");
            for (domain, count) in top_domains {
                println!("{}: {} times", domain, count);
            }

            println!("\nTop Blocked Domains:");
            for (ip, count) in top_ips {
                println!("{}: {} times", ip, count);
            }

            Ok(())

        })?;

        Ok(())
    }
}