use pyo3::prelude::*;
use pyo3::types::*;
use wcore::models::WTokens;
use std::ffi;
use std::ptr::addr_of;
use wcore;
use wcore::models::{Range, Token::*};

fn create_pylist<'a>(py: Python, acc: &'a PyList, wcode: Vec<WTokens>) -> &'a PyList {
     for section in wcode {
        for code_result in section {
            match code_result {
                Container(x) | ContainerLiteral(x) | Atom(x) | Special(x) => acc.append(x).unwrap(),
                Value(x) => acc.append(x).unwrap(),
                Function(x) | FunctionLiteral(x) => {
                    let address: *const i8 = addr_of!(x).cast();
                    let slice = unsafe { ffi::CStr::from_ptr(address) };
                    acc.append(slice.to_string_lossy().into_owned()).unwrap()
                }
                Parameter(x) => acc.append(match x {
                    Range::Full(y) => format!("{}..={}", y.start(), y.end()),
                    Range::To(y) => format!("..{}", y.start),
                    Range::From(y) => format!("{}..", y.end),
                }).unwrap(),
                Group(x) => acc.append(create_pylist(py, PyList::empty(py), vec![x])).unwrap(),
            };
        }
    }

    acc
}

#[pyfunction]
fn eval(py: Python, code: String) -> PyResult<&PyAny> {
    let result = PyList::empty(py);
    let code_eval = wcore::eval(&code);

    create_pylist(py, result, code_eval);

    Ok(result.downcast()?)
}

#[pymodule]
fn wcore_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}
