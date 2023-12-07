use ::gamma_chess::model::Dataset;
/// Refer to the sample implementation:
/// https://github.com/LaurentMazare/tch-ext
use pyo3::prelude::*;
use pyo3_tch::PyTensor;

#[pyclass]
struct PyDataset {
    dataset: Dataset,
}

#[pymethods]
impl PyDataset {
    #[new]
    fn new(file_path: String) -> Self {
        PyDataset {
            dataset: Dataset::new(&file_path),
        }
    }

    #[getter]
    fn positions(&self) -> PyResult<PyTensor> {
        Ok(PyTensor(self.dataset.positions.copy()))
    }

    #[getter]
    fn moves(&self) -> PyResult<PyTensor> {
        Ok(PyTensor(self.dataset.moves.copy()))
    }
}

#[pymodule]
fn gamma_chess(py: Python, m: &PyModule) -> PyResult<()> {
    py.import("torch")?;
    m.add_class::<PyDataset>()?;
    // Add other classes or functions here if needed
    Ok(())
}
