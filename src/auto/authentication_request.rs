// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use AuthenticationScheme;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use Credential;
use ffi;
use glib;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest, ffi::WebKitAuthenticationRequestClass>);

    match fn {
        get_type => || ffi::webkit_authentication_request_get_type(),
    }
}

pub trait AuthenticationRequestExt {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self);

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<String>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<String>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthenticationRequest> + IsA<glib::object::Object>> AuthenticationRequestExt for O {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32 {
        unsafe {
            ffi::webkit_authentication_request_get_port(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_proposed_credential(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::WebKitAuthenticationRequest, f: glib_ffi::gpointer)
where P: IsA<AuthenticationRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AuthenticationRequest::from_glib_borrow(this).downcast_unchecked())
}
