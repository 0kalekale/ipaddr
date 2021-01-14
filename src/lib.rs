use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use curl::easy::Easy;

fn _getip() -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url("https://httpbin.org/ip").unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    return String::from_utf8(data.clone()).unwrap();
}
#[pyfunction]
fn __getip() -> PyResult<String> {
    let mut ret = _getip();

    Ok(ret)
        
}

#[pymodule]
fn ipaddr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__getip, m)?)?;

    Ok(())
}