use proc_maps::get_process_maps;
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyDict},
};
use std::collections::HashMap;
//use scanflow::value_scanner::ValueScanner;
//use memflow::prelude::v1::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn get_mem_maps(pid_int: i32) -> PyResult<Py<PyDict>> {
    let maps = get_process_maps(pid_int)?;
    let mut resp = HashMap::<String, String>::new();
    for map in maps {
        //println!("Filename {:?} Address {} Size {}", map.filename(), map.start(), map.size());
        let k = format!("Addr:{:x}:{:x}", map.start(), map.start() + map.size());
        let v = format!("{:?}", map.filename());
        resp.insert(k, v);
    }
    Python::with_gil(|py| -> PyResult<Py<PyDict>> { Ok(resp.into_py_dict(py).into_py(py)) })
}

#[pyclass]
struct PyScanner {
    //process: Process,
    //value_scanner: ValueScanner
}

#[pymethods]
impl PyScanner {
    #[new]
    fn new(pid: Option<u32>) -> Self {
        PyScanner {
            //process: Process{},
            //value_scanner: Default::default()
        }
    }

}

#[pyfunction]
fn gen_scanner() -> PyResult<Py<PyScanner>> {
    Python::with_gil(|py| {
        return Py::new(py, PyScanner::new(None));
    })
}

#[pyfunction]
fn test_binary(binary: &[u8]) -> PyResult<&[u8]> {
    println!("Got test: {:?}", binary);
    Ok(binary)
}

/// A Python module implemented in Rust.
#[pymodule]
fn medit_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_mem_maps, m)?)?;
    m.add_function(wrap_pyfunction!(test_binary, m)?)?;
    m.add_function(wrap_pyfunction!(gen_scanner, m)?)?;
    Ok(())
}
