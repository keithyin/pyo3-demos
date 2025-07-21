use numpy::{PyArray1, PyReadonlyArray2, PyReadwriteArray2};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn matrix_point_product_and_sum(arr: PyReadonlyArray2<f64>) -> f64 {
    let arr = arr.as_array();
    arr.map(|&x| (x * x)).sum()
}

#[pyfunction]
fn square_in_place(mut arr: PyReadwriteArray2<f64>) {
    let mut arr = arr.as_array_mut();
    arr.mapv_inplace(|v| v * v);
}

// rust创建 ndarray，并交由python管理内存
// #[pyfunction]
// fn generate_new_ndarray<'py>(m: &Bound<'py, PyModule>) -> Bound<'py, PyArray1<f64>> {
//     let arr = numpy::ndarray::Array1::from_elem((1000,), 10.0f64);

//     // 如果想要避免考虑copy。使用  from_owned_object_array
//     let arr: Bound<'_, numpy::PyArray<f64, numpy::ndarray::Dim<[usize; 1]>>> =
//         PyArray1::from_array(m.py(), &arr);

//     arr
// }

#[pyfunction]
fn generate_new_ndarray<'py>(py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
    let arr = numpy::ndarray::Array1::from_elem((1000,), 10.0f64);

    // 如果想要避免考虑copy。使用  from_owned_object_array
    let arr: Bound<'_, numpy::PyArray<f64, numpy::ndarray::Dim<[usize; 1]>>> =
        PyArray1::from_array(py, &arr);

    arr
}

#[pyfunction]
fn matrix_point_product_and_sum_without_gil(py: Python<'_>, arr: PyReadonlyArray2<f64>) -> f64 {
    let arr = arr.as_array();
    py.allow_threads(|| arr.map(|&x| (x * x)).sum())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_demos(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_point_product_and_sum, m)?)?;
    m.add_function(wrap_pyfunction!(
        matrix_point_product_and_sum_without_gil,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(generate_new_ndarray, m)?)?;

    m.add_function(wrap_pyfunction!(square_in_place, m)?)?;

    Ok(())
}
