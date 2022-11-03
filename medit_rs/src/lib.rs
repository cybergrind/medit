use memflow::plugins::os::IntoProcessInstance;
use memflow::prelude::v1::*;
use proc_maps::get_process_maps;
use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyDict},
};
use scanflow::value_scanner::ValueScanner;
use std::collections::HashMap;

use std::prelude::v1::*;

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

#[pyclass(unsendable)]
struct PyScanner {
    process: &'static mut TT,
    scanner: &'static mut ValueScanner,
}

type TT =
    IntoProcessInstance<'static, CBox<'static, trait_group::c_void>, CArc<trait_group::c_void>>;

#[pymethods]
impl PyScanner {
    #[new]
    fn new(pid: Option<u32>) -> Self {
        let chain = OsChain::new([].into_iter(), [(6, "native")].into_iter()).unwrap();
        let inv = Inventory::scan();
        let os = inv.builder().os_chain(chain).build().unwrap();
        let proc = os.into_process_by_pid(pid.unwrap()).unwrap();
        PyScanner {
            process: Box::leak(Box::new(proc)),
            scanner: Box::leak(Box::new(ValueScanner::default())),
        }
    }

    fn read(&mut self, addr: i64, size: Option<usize>) -> PyResult<Vec<u8>> {
        let resp = self
            .process
            .read_raw(Address::from(addr), size.unwrap_or(8));
        Ok(resp.unwrap())
    }

    fn search(&mut self, pattern: &[u8]) -> PyResult<Vec<u64>> {
        self.scanner.reset();
        self.scanner.scan_for(self.process, pattern).unwrap();
        return Ok(self.scanner
            .matches()
            .iter()
            .map(|x| x.to_umem())
            .collect());
    }

    fn filter(&mut self, pattern: &[u8]) -> PyResult<Vec<u64>> {
        self.scanner.scan_for(self.process, pattern).unwrap();
        return Ok(self.scanner
            .matches()
            .iter()
            .map(|x| x.to_umem())
            .collect());
    }

    fn write(&mut self, addr: i64, data: &[u8]) -> PyResult<()> {
        self.process.write_raw(Address::from(addr), data).unwrap();
        return Ok(());
    }
}

#[pyfunction]
fn test_binary(binary: &[u8]) -> PyResult<&[u8]> {
    println!("Got test: {:?}", binary);
    Ok(binary)
}

/// A Python module implemented in Rust.
#[pymodule]
fn medit_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_mem_maps, m)?)?;
    m.add_function(wrap_pyfunction!(test_binary, m)?)?;
    m.add_class::<PyScanner>()?;
    Ok(())
}
