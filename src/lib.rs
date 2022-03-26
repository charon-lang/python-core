use pyo3::prelude::*;
use pyo3::types::*;
use std::ffi;
use std::ptr::addr_of;
use wcore;
use wcore::models::{Range, Token::*};

#[pyfunction]
fn eval(py: Python, code: String) -> PyResult<&PyAny> {
    let result = PyList::empty(py);
    let code_eval = wcore::eval(&code);

    for section in code_eval {
        for code_result in section {
            match code_result {
                Container(x) | ContainerLiteral(x) | Atom(x) | Special(x) => result.append(x),
                Value(x) => result.append(x),
                Function(x) | FunctionLiteral(x) => {
                    let address: *const i8 = addr_of!(x).cast();
                    let slice = unsafe { ffi::CStr::from_ptr(address) };
                    result.append(slice.to_string_lossy().into_owned())
                }
                Parameter(x) => result.append(match x {
                    Range::Full(y) => format!("{}..={}", y.start(), y.end()),
                    Range::To(y) => format!("..{}", y.start),
                    Range::From(y) => format!("{}..", y.end),
                }),
                _ => unimplemented!(),
            };
        }
    }

    Ok(result.downcast()?)
}

#[pymodule]
fn wcore_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}
