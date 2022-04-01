use pyo3::prelude::*;
use pyo3::types::*;
use std::ffi;
use std::ptr::addr_of;

use wcore::{
    evaluator::WEval,
    models::{Range, State, Token::*, WTokens},
};

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
                Parameter(x) => acc
                    .append(match x {
                        Range::Full(y) => format!("{}..={}", y.start(), y.end()),
                        Range::To(y) => format!("..{}", y.start),
                        Range::From(y) => format!("{}..", y.end),
                    })
                    .unwrap(),
                Group(x) => acc
                    .append(create_pylist(py, PyList::empty(py), vec![x]))
                    .unwrap(),
            };
        }
    }

    acc
}

#[pyclass(name = "State")]
struct PyState {
    state: State,
}

#[pymethods]
impl PyState {
    #[new]
    fn __new__() -> PyResult<Self> {
        Ok(Self {
            state: State::new(),
        })
    }

    fn apply<'a>(&mut self, py: Python<'a>, code: String) -> PyResult<&'a PyAny> {
        let result = PyList::empty(py);
        let code_eval = self.state.apply(&code);

        create_pylist(py, result, code_eval);
        Ok(result.downcast()?)
    }
}

#[pymodule]
fn wcore_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyState>()?;
    Ok(())
}
