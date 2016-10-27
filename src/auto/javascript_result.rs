// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use java_script_core;

glib_wrapper! {
    pub struct JavascriptResult(Shared<ffi::WebKitJavascriptResult>);

    match fn {
        ref => |ptr| ffi::webkit_javascript_result_ref(ptr),
        unref => |ptr| ffi::webkit_javascript_result_unref(ptr),
    }
}

impl JavascriptResult {

    pub fn get_global_context(&self) -> Option<java_script_core::GlobalContext> {
        unsafe {
            from_glib_full(ffi::webkit_javascript_result_get_global_context(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<java_script_core::Value> {
        unsafe {
            from_glib_full(ffi::webkit_javascript_result_get_value(self.to_glib_none().0))
        }
    }
}
