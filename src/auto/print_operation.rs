// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use WebView;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct PrintOperation(Object<ffi::WebKitPrintOperation>);

    match fn {
        get_type => || ffi::webkit_print_operation_get_type(),
    }
}

impl PrintOperation {
    pub fn new(web_view: &WebView) -> PrintOperation {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_print_operation_new(web_view.to_glib_none().0))
        }
    }

    //pub fn get_page_setup(&self) -> /*Ignored*/Option<gtk::PageSetup> {
    //    unsafe { TODO: call ffi::webkit_print_operation_get_page_setup() }
    //}

    //pub fn get_print_settings(&self) -> /*Ignored*/Option<gtk::PrintSettings> {
    //    unsafe { TODO: call ffi::webkit_print_operation_get_print_settings() }
    //}

    pub fn print(&self) {
        unsafe {
            ffi::webkit_print_operation_print(self.to_glib_none().0);
        }
    }

    //pub fn run_dialog<T: IsA</*Ignored*/gtk::Window>>(&self, parent: Option<&T>) -> /*Ignored*/PrintOperationResponse {
    //    unsafe { TODO: call ffi::webkit_print_operation_run_dialog() }
    //}

    //pub fn set_page_setup(&self, page_setup: /*Ignored*/&gtk::PageSetup) {
    //    unsafe { TODO: call ffi::webkit_print_operation_set_page_setup() }
    //}

    //pub fn set_print_settings(&self, print_settings: /*Ignored*/&gtk::PrintSettings) {
    //    unsafe { TODO: call ffi::webkit_print_operation_set_print_settings() }
    //}

    //pub fn connect_failed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored error: GLib.Error
    //}

    pub fn connect_finished<F: Fn(&PrintOperation) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&PrintOperation) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn finished_trampoline(this: *mut ffi::WebKitPrintOperation, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&PrintOperation) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
