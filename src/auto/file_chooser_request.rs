// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct FileChooserRequest(Object<ffi::WebKitFileChooserRequest>);

    match fn {
        get_type => || ffi::webkit_file_chooser_request_get_type(),
    }
}

impl FileChooserRequest {
    pub fn cancel(&self) {
        unsafe {
            ffi::webkit_file_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    pub fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_mime_types(self.to_glib_none().0))
        }
    }

    //pub fn get_mime_types_filter(&self) -> /*Ignored*/Option<gtk::FileFilter> {
    //    unsafe { TODO: call ffi::webkit_file_chooser_request_get_mime_types_filter() }
    //}

    pub fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_file_chooser_request_get_select_multiple(self.to_glib_none().0))
        }
    }

    pub fn get_selected_files(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_selected_files(self.to_glib_none().0))
        }
    }

    pub fn select_files(&self, files: &[&str]) {
        unsafe {
            ffi::webkit_file_chooser_request_select_files(self.to_glib_none().0, files.to_glib_none().0);
        }
    }
}
