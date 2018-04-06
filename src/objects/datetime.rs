use err::PyResult;
use object::PyObject;
use std::os::raw::c_int;
use ffi::{PyDateTimeAPI};
use python::{Python, ToPyPointer};
use instance::Py;



// datetime.date bindings
pub struct PyDate(PyObject);
pyobject_convert!(PyDate);
pyobject_nativetype!(PyDate, PyDateTime_DateType, PyDate_Check);

impl PyDate {
    pub fn new(py: Python, year: u32, month: u32, day: u32) -> PyResult<Py<PyDate>> {
        let y = year as c_int;
        let m = month as c_int;
        let d = day as c_int;

        unsafe {
            let ptr = PyDateTimeAPI.Date_FromDate.unwrap()(y, m, d, PyDateTimeAPI.DateType);
            Py::from_owned_ptr_or_err(py, ptr)
        }
    }
}


// datetime.datetime bindings
pub struct PyDateTime(PyObject);
pyobject_convert!(PyDateTime);
pyobject_nativetype!(PyDateTime, PyDateTime_DateTimeType, PyDateTime_Check);


impl PyDateTime {
    pub fn new(py: Python, year: u32, month: u32, day: u32,
               hour: u32, minute: u32, second: u32, microsecond: u32,
               tzinfo: &PyObject) -> PyResult<Py<PyDateTime>> {
        let y = year as c_int;
        let mo = month as c_int;
        let d = day as c_int;
        let h = hour as c_int;
        let mi = minute as c_int;
        let s = second as c_int;
        let u = microsecond as c_int;

        unsafe {
            let ptr = PyDateTimeAPI.DateTime_FromDateAndTime.unwrap()(
                y, mo, d, h, mi, s, u, tzinfo.as_ptr(),
                PyDateTimeAPI.DateTimeType
            );
            Py::from_owned_ptr_or_err(py, ptr)
        }
    }
}

// datetime.tzinfo bindings
pub struct PyTzInfo(PyObject);
pyobject_convert!(PyTzInfo);
pyobject_nativetype!(PyTzInfo, PyDateTime_TZInfoType, PyTZInfo_Check);
