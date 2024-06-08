use pyo3::{prelude::*, types::{IntoPyDict, PyModule}};
use std::io::Read;

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(filepath)?;
    Ok(data)
}

fn main() -> PyResult<()> {
    let filepath = "LogReducer/hello.py";
    let code = read_file_string(filepath).unwrap();
    Python::with_gil(|py| {
    let activators = PyModule::from_code(py, &code, "activators.py", "activators")?;

    let relu_result: f64 = activators.getattr("relu")?.call1((-1.0,))?.extract()?;
    assert_eq!(relu_result, 0.0);

    let kwargs = [("slope", 0.2)].into_py_dict(py);
    let lrelu_result: f64 = activators
        .getattr("leaky_relu")?.call((-1.0,), Some(kwargs))?
        .extract()?;
    assert_eq!(lrelu_result, -0.2);
    let hello_world = activators.getattr("hello_world")?.call0()?;
    Ok(())
})
}

