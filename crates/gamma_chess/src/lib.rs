#![feature(vec_into_raw_parts)]
use std::os::raw::{c_int, c_void};

use ndarray::Array;
use tch::{Device, Kind, Tensor};
use torch_sys::at_tensor_of_blob;

pub mod archive;
pub mod etl;
pub mod model;
pub mod proto;

pub fn kind_to_c_int(kind: Kind) -> c_int {
    // These values should be in sync with include/c10/core/ScalarType.h
    match kind {
        Kind::Uint8 => 0,
        Kind::Int8 => 1,
        Kind::Int16 => 2,
        Kind::Int => 3,
        Kind::Int64 => 4,
        Kind::Half => 5,
        Kind::Float => 6,
        Kind::Double => 7,
        Kind::ComplexHalf => 8,
        Kind::ComplexFloat => 9,
        Kind::ComplexDouble => 10,
        Kind::Bool => 11,
        Kind::QInt8 => 12,
        Kind::QUInt8 => 13,
        Kind::QInt32 => 14,
        Kind::BFloat16 => 15,
    }
}

pub fn device_to_c_int(device: Device) -> c_int {
    match device {
        Device::Cpu => -1,
        Device::Cuda(device_index) => device_index as c_int,
        Device::Mps => -2,
        Device::Vulkan => -3,
    }
}

unsafe fn ndarray_to_tensor(array: Array<f32, ndarray::IxDyn>) -> Tensor {
    println!("a");
    let shape: Vec<i64> = array.shape().iter().map(|&s| s as i64).collect();
    let strides: Vec<i64> = array.strides().iter().map(|&s| s as i64).collect();
    let data_ptr = array.as_ptr();

    println!("b");
    // Calculate the byte length of the array
    let num_bytes = array.len() * std::mem::size_of::<f32>();

    println!("c");
    // Create a byte slice from the f32 data
    let byte_slice = std::slice::from_raw_parts(data_ptr as *const u8, num_bytes);

    println!("d");
    // Ensure the ndarray is not dropped while the Tensor exists
    std::mem::forget(array);

    println!("e");
    // Get the raw pointer of the byte slice
    let byte_slice_ptr = byte_slice.as_ptr();

    println!("f");
    Tensor::from_blob(byte_slice_ptr, &shape, &strides, Kind::Float, Device::Cpu)
}

unsafe fn ndarray_to_tensor_2(mut array: Array<f32, ndarray::IxDyn>) -> Tensor {
    let shape: Vec<i64> = array.shape().iter().map(|&s| s as i64).collect();
    let strides: Vec<i64> = array
        .strides()
        .iter()
        .map(|&s| s as i64 * std::mem::size_of::<f32>() as i64)
        .collect();
    let data_ptr = array.as_mut_ptr() as *mut c_void;

    // Take ownership of the data to prevent ndarray from deallocating it
    let array_data = Box::from_raw(array.as_mut_ptr());

    // Create a C_tensor from the raw data
    let c_tensor = at_tensor_of_blob(
        data_ptr,
        shape.as_ptr(),
        shape.len(),
        strides.as_ptr(),
        strides.len(),
        kind_to_c_int(Kind::Float),
        device_to_c_int(Device::Cpu),
    );

    // Convert the C_tensor to a Tensor
    let tensor = Tensor::from_ptr(c_tensor);

    // Hand over the responsibility of deallocating memory to the Tensor
    std::mem::forget(array_data);

    tensor
}
// // Function to convert an ndarray to a Tensor without copying
// fn array_to_tensor_no_copy<T, D>(array: Array<T, D>) -> Tensor
// where
//     T: ndarray::LinalgScalar,
//     D: ndarray::Dimension,
// {
//     // Ensure that the array is in standard layout
//     let array = if array.is_standard_layout() {
//         array
//     } else {
//         array.to_owned()
//     };

//     // Get the shape and strides of the array
//     let shape = array.shape();

//     // A stride is the step size to move from one element to the next in memory for each dimension.
//     // It's expressed in terms of the number of elements, not bytes.
//     // We multiply the stride by the size of a single element (T) in bytes. For instance,
//     // if T is f32, each element is 4 bytes.
//     let strides: Vec<i64> = array
//         .strides()
//         .iter()
//         .map(|&s| s as i64 * std::mem::size_of::<T>() as i64)
//         .collect();

//     // Take ownership of the array's data
//     // let (data, _length, _capacity) = array.into_raw_vec().into_raw_parts();

//     let data = array.into_raw_vec();
//     let tensor =
//         unsafe { Tensor::f_of_data(data.as_slice(), &shape, &strides, Kind::Float).unwrap() };

//     // Create a tensor from the raw parts
//     unsafe { Tensor::from_data(data, &shape, &strides) }
// }

#[test]
fn from_ndarray_f64() {
    let nd = ndarray::arr2(&[[1f32, 2.], [3., 4.]]);
    let tensor = Tensor::try_from(nd.clone()).unwrap();
    let tensor_2 = unsafe { ndarray_to_tensor(nd.clone().into_dyn()) };
    let tensor_3 = unsafe { ndarray_to_tensor_2(nd.clone().into_dyn()) };

    assert_eq!(tensor, tensor_2);
    // assert_eq!(Vec::<f64>::from(tensor).as_slice(), nd.as_slice().unwrap());
}
