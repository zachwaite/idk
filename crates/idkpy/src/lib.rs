use pyo3::prelude::*;
use rpgle_parser::{parse_ast, parse_cst};
use serde_json;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Parses an input string and returns a json string AST
#[pyfunction]
fn parse_rpgle(txt: &str) -> PyResult<String> {
    if let Ok(cst) = parse_cst(txt) {
        if let Ok(ast) = parse_ast(&cst) {
            let out = serde_json::to_string(&ast).expect("this derives serde so must unwrap");
            return Ok(out);
        }
    }
    Ok("???".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn idkpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_rpgle, m)?)?;
    Ok(())
}
