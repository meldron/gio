// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use IOStream;
use SocketConnection;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TcpConnection(Object<ffi::GTcpConnection, ffi::GTcpConnectionClass>): SocketConnection, IOStream;

    match fn {
        get_type => || ffi::g_tcp_connection_get_type(),
    }
}

pub trait TcpConnectionExt {
    fn get_graceful_disconnect(&self) -> bool;

    fn set_graceful_disconnect(&self, graceful_disconnect: bool);

    fn connect_property_graceful_disconnect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TcpConnection> + IsA<glib::object::Object>> TcpConnectionExt for O {
    fn get_graceful_disconnect(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tcp_connection_get_graceful_disconnect(self.to_glib_none().0))
        }
    }

    fn set_graceful_disconnect(&self, graceful_disconnect: bool) {
        unsafe {
            ffi::g_tcp_connection_set_graceful_disconnect(self.to_glib_none().0, graceful_disconnect.to_glib());
        }
    }

    fn connect_property_graceful_disconnect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::graceful-disconnect",
                transmute(notify_graceful_disconnect_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_graceful_disconnect_trampoline<P>(this: *mut ffi::GTcpConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TcpConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TcpConnection::from_glib_borrow(this).downcast_unchecked())
}
