use pyo3::prelude::*;
//use process_memory::{Memory, DataMember, Pid, TryIntoProcessHandle};
use proc_maps::get_process_maps;



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn get_process_handle(pid_int: i32) -> PyResult<String> {
    let maps = get_process_maps(pid_int)?;
    for map in maps {
        println!("Filename {:?} Address {} Size {}", map.filename(), map.start(), map.size());
    }
    return Ok("Ok".to_string());
}

#[pyfunction]
fn test_binary(binary: &[u8]) -> PyResult<&[u8]> {
    println!("Got test: {:?}", binary);
    return Ok(binary)
}

/// A Python module implemented in Rust.
#[pymodule]
fn medit_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_process_handle, m)?)?;
    m.add_function(wrap_pyfunction!(test_binary, m)?)?;
    Ok(())
}
