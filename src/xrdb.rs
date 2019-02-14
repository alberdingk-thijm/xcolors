use crate::x11api::XDisplay;
use x11::xlib::{XrmDatabase, XrmValue, XResourceManagerString, XrmGetResource, XrmGetStringDatabase, XrmDestroyDatabase};
use std::ffi::{CString, CStr};
use libc;
use scopeguard::defer;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Xrdb {
    display: XDisplay,
    db: BTreeMap<String, String>,
}

impl Xrdb {
}

impl Drop for Xrdb {
    fn drop(&mut self) {
    }
}

unsafe fn get_xrm_resource<'a>(db: XrmDatabase, name: &'a str) -> Option<&'a str> {
    let mut value = XrmValue {
        size: 0,
        addr: std::ptr::null_mut(),
    };

    let mut value_type: *mut libc::c_char = std::ptr::null_mut();
    let name_c_str = CString::new(name).unwrap();
    let c_str = CString::new("String").unwrap();
    if XrmGetResource(
        db,
        name_c_str.as_ptr(),
        c_str.as_ptr(),
        &mut value_type,
        &mut value
    ) != 0 && !value.addr.is_null() {
        let value_addr: &CStr = CStr::from_ptr(value.addr);
        value_addr.to_str().ok()
    } else {
        None
    }
}

