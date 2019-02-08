use crate::x11api::XDisplay;
use x11::xlib::{XrmDatabase, XrmValue, XResourceManagerString, XrmGetResource, XrmGetStringDatabase, XrmDestroyDatabase};
use std::ffi::{CString, CStr};
use libc;
use scopeguard::defer;

/// A struct for representing the colours found in the X Resource Manager's database.
#[derive(Clone, Debug, Default)]
pub struct XColors {
    /// Foreground colour: matches the "foreground" key.
    pub fg: Option<String>,
    /// Background colour: matches the "background" key.
    pub bg: Option<String>,
    /// Cursor colour: matches the "cursorColor" key.
    pub cursor: Option<String>,
    /// Colours 0 to 15: match the "color{}" keys, where {} is a number from 0 to 15 inclusive.
    pub colors: [Option<String>; 16],
}

impl XColors {
    /// Return an XColors struct if one can be obtained from the X Resource Manager.
    /// The prefix specifies the part of the database tree to look under.
    /// Giving the prefix "xterm" will return a struct filled with the values of the keys
    ///
    /// * xterm.foreground
    /// * xterm.background
    /// * xterm.cursorColor
    /// * xterm.color1 through xterm.color15
    ///
    /// If any of those keys are missing, the corresponding field will be set to None.
    pub fn new<'a>(prefix: &'a str) -> Option<Self> {
        let display = XDisplay::new()
            .expect("Failed to acquire X display!");
        unsafe {
            let rms = XResourceManagerString(*display);
            if !rms.is_null() {
                let db = XrmGetStringDatabase(rms);
                if !db.is_null() {
                    defer!({
                        XrmDestroyDatabase(db);
                    });
                    return Some(XColors::from_database(db, prefix));
                }
            }
        }
        None
    }

    unsafe fn from_database<'a>(db: XrmDatabase, prefix: &'a str) -> Self {
        let mut xcolors = XColors::default();
        let fg_str = format!("{}.foreground", prefix);
        let bg_str = format!("{}.background", prefix);
        let cursor_str = format!("{}.cursorColor", prefix);
        let fg = get_xrm_resource(db, &fg_str).map(|s| String::from(s));
        let bg = get_xrm_resource(db, &bg_str).map(|s| String::from(s));
        let cursor = get_xrm_resource(db, &cursor_str).map(|s| String::from(s));
        let color_names = (0..16).map(|i| format!("{}.color{}", prefix, i));
        let colors = color_names.map(|s| get_xrm_resource(db, &s).map(|s| String::from(s))).collect::<Vec<_>>();
        xcolors.fg = fg;
        xcolors.bg = bg;
        xcolors.cursor = cursor;
        xcolors.colors = [
            colors[0].clone(), colors[1].clone(), colors[2].clone(), colors[3].clone(),
            colors[4].clone(), colors[5].clone(), colors[6].clone(), colors[7].clone(),
            colors[8].clone(), colors[9].clone(), colors[10].clone(), colors[11].clone(),
            colors[12].clone(), colors[13].clone(), colors[14].clone(), colors[15].clone(),
        ];
        xcolors
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

