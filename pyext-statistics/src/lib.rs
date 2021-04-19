use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn mean(data: &PyList) -> PyResult<f64> {
    if data.len() == 0 {
        Err(PyRuntimeError::new_err("Something is wrong"))
    } else {
        let mut sum = 0f64;

        for py_object in data.iter() {
            let r: f64 = py_object.extract().unwrap();
            sum += r;
        }

        Ok(sum / data.len() as f64)
    }
}

#[pyfunction]
fn fmean(data: &PyList) -> PyResult<f64> {
    if data.len() == 0 {
        Err(PyRuntimeError::new_err("Something is wrong"))
    } else {
        let mut sum = 0f64;

        for py_object in data.iter() {
            let r: f64 = py_object.extract().unwrap();
            sum += r;
        }

        Ok(sum / data.len() as f64)
    }
}

#[pyfunction]
fn geometric_mean(data: &PyList) -> PyResult<f64> {
    if data.len() == 0 {
        Err(PyRuntimeError::new_err("Something is wrong"))
    } else {
        let mut prod = 1.0f64;

        for py_object in data.iter() {
            let r: f64 = py_object.extract().unwrap();
            prod *= r;
        }

        Ok(prod.powf(1.0f64 / data.len() as f64))
    }
}

#[pyfunction]
fn harmonic_mean(data: &PyList) -> PyResult<f64> {
    if data.len() == 0 {
        Err(PyRuntimeError::new_err("Something is wrong"))
    } else {
        let mut sum = 0f64;

        for py_object in data.iter() {
            let r: f64 = py_object.extract().unwrap();
            sum += 1f64 / r;
        }

        Ok(data.len() as f64 / sum)
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn stats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(fmean, m)?)?;
    m.add_function(wrap_pyfunction!(geometric_mean, m)?)?;
    m.add_function(wrap_pyfunction!(harmonic_mean, m)?)?;

    Ok(())
}