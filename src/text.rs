/* Copyright 2016 Jordan Miner
 *
 * Licensed under the MIT license <LICENSE or
 * http://opensource.org/licenses/MIT>. This file may not be copied,
 * modified, or distributed except according to those terms.
 */

use super::control_prelude::*;
use std::borrow::Cow;

#[derive(Clone)]
pub struct Text(HandleRc);

impl Text {
    pub fn new() -> Self {
        unsafe {
            ::iup_open();
            let ih = IupText(ptr::null_mut());
            Text(HandleRc::new(ih))
        }
    }

    pub fn value(&self) -> String {
        get_str_attribute(self.handle(), "VALUE\0")
    }
    pub unsafe fn value_slice(&self) -> Cow<str> {
        get_str_attribute_slice(self.handle(), "VALUE\0")
    }

    pub fn set_value(&self, value: &str) -> &Self {
        set_str_attribute(self.handle(), "VALUE\0", value);
        self
    }

    /// Inserts text at the end of the current text. Ignored if called before the control is mapped.
    /// If this control is multiline, [`append_newline`](./struct.Text.html#method.append_newline)
    /// is true, and the current text is not empty, then a `'\n'` character will be automatically
    /// inserted before the appended text.
    pub fn append(&self, text: &str) -> &Self {
        set_str_attribute(self.handle(), "APPEND\0", text);
        self
    }

    /// If true, this control is multiline, and the current text is not empty, then a `'\n'`
    /// character will be automatically inserted before text appended using `append`.
    ///
    /// The default is true.
    pub fn append_newline(&self) -> bool {
        unsafe {
            get_str_attribute_slice(self.handle(), "APPENDNEWLINE\0") == "YES"
        }
    }

    /// See [`append_newline`](./struct.Text.html#method.append_newline)
    pub fn set_append_newline(&self, enabled: bool) -> &Self {
        set_str_attribute(self.handle(), "APPENDNEWLINE\0", if enabled { "YES\0" } else { "NO\0" });
        self
    }

    pub fn multiline(&self) -> bool {
        unsafe {
            get_str_attribute_slice(self.handle(), "MULTILINE\0") == "YES"
        }
    }

    pub fn set_multiline(&self, multiline: bool) -> &Self {
        set_str_attribute(self.handle(), "MULTILINE\0", if multiline { "YES\0" } else { "NO\0" });
        self
    }
}

impl_control_traits!(Text);

impl ActiveAttribute for Text {}
impl CanFocusAttribute for Text {}
impl ExpandAttribute for Text {}
impl MinMaxSizeAttribute for Text {}
impl ScrollbarAttribute for Text {}
impl TipAttribute for Text {}
impl VisibleAttribute for Text {}
impl VisibleColumnsLinesAttribute for Text {}

impl MenuCommonCallbacks for Text {}
impl GetKillFocusCallbacks for Text {}
impl EnterLeaveWindowCallbacks for Text {}
impl ValueChangedCallback for Text {}

#[derive(Clone)]
pub struct CaretArgs {
    pub lin: usize,
    pub col: usize,
    pub pos: usize,
    _dummy: (),
}

impl_callbacks! {
    Text {
        "CARET_CB\0" => caret_event {
            CARET_CALLBACKS<FnMut(&CaretArgs), CaretCallbackToken>
        }
        unsafe extern fn caret_cb(ih: *mut Ihandle, lin: c_int, col: c_int, pos: c_int) -> c_int {
            with_callbacks(ih, &CARET_CALLBACKS, |cbs| {
                let args = CaretArgs {
                    lin: lin as usize,
                    col: col as usize,
                    pos: pos as usize,
                    _dummy: (),
                };
                for cb in cbs {
                    (&mut *cb.1.borrow_mut())(&args);
                }
                IUP_DEFAULT
            })
        }
    }
}
