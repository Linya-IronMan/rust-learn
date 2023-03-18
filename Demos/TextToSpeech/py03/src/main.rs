use pyo3::prelude::*;
use pyo3::Python;

fn main() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let hello_module = py.import("hello")?;
    let say_hello = hello_module.getattr("say_hello")?;
    say_hello.call1(py, ("World",))?;

    Ok(())
}

