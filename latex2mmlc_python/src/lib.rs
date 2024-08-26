use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyString;

use latex2mmlc::{latex_to_mathml, Arena, Display};

create_exception!(_latex2mmlc_rust, LatexError, PyException);

/// Convert LaTeX equation to MathML.
#[pyfunction]
fn convert_latex<'a>(
    py: Python<'a>,
    latex: &str,
    block: bool,
    pretty: bool,
) -> PyResult<Bound<'a, PyString>> {
    let arena = Arena::new();
    let result = latex_to_mathml(
        latex,
        &arena,
        if block {
            Display::Block
        } else {
            Display::Inline
        },
        pretty,
    )
    .map_err(|latex_error| LatexError::new_err(latex_error.to_string()))?;
    Ok(PyString::new_bound(py, &result))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _latex2mmlc_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("LatexError", m.py().get_type_bound::<LatexError>())?;
    m.add_function(wrap_pyfunction!(convert_latex, m)?)?;
    Ok(())
}
