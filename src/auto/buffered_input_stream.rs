// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use FilterInputStream;
use InputStream;
use Seekable;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
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
    pub struct BufferedInputStream(Object<ffi::GBufferedInputStream, ffi::GBufferedInputStreamClass>): FilterInputStream, InputStream, Seekable;

    match fn {
        get_type => || ffi::g_buffered_input_stream_get_type(),
    }
}

impl BufferedInputStream {
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new(base_stream.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_sized<P: IsA<InputStream>>(base_stream: &P, size: usize) -> BufferedInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_buffered_input_stream_new_sized(base_stream.to_glib_none().0, size)).downcast_unchecked()
        }
    }
}

pub trait BufferedInputStreamExt: Sized {
    fn fill<'a, P: Into<Option<&'a Cancellable>>>(&self, count: isize, cancellable: P) -> Result<isize, Error>;

    fn fill_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: isize, io_priority: glib::Priority, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn fill_async_future(&self, count: isize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, isize), Error = (Self, Error)>>;

    fn get_available(&self) -> usize;

    fn get_buffer_size(&self) -> usize;

    fn peek_buffer(&self) -> Vec<u8>;

    fn read_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i32, Error>;

    fn set_buffer_size(&self, size: usize);

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BufferedInputStream> + IsA<glib::object::Object> + Clone + 'static> BufferedInputStreamExt for O {
    fn fill<'a, P: Into<Option<&'a Cancellable>>>(&self, count: isize, cancellable: P) -> Result<isize, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_fill(self.to_glib_none().0, count, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn fill_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<isize, Error>) + Send + 'static>(&self, count: isize, io_priority: glib::Priority, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn fill_async_trampoline<Q: FnOnce(Result<isize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_fill_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = fill_async_trampoline::<Q>;
        unsafe {
            ffi::g_buffered_input_stream_fill_async(self.to_glib_none().0, count, io_priority.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn fill_async_future(&self, count: isize, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, isize), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.fill_async(
                 count,
                 io_priority,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn get_available(&self) -> usize {
        unsafe {
            ffi::g_buffered_input_stream_get_available(self.to_glib_none().0)
        }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe {
            ffi::g_buffered_input_stream_get_buffer_size(self.to_glib_none().0)
        }
    }

    fn peek_buffer(&self) -> Vec<u8> {
        unsafe {
            let mut count = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(ffi::g_buffered_input_stream_peek_buffer(self.to_glib_none().0, &mut count), count as usize);
            ret
        }
    }

    fn read_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i32, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_buffered_input_stream_read_byte(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_buffer_size(&self, size: usize) {
        unsafe {
            ffi::g_buffered_input_stream_set_buffer_size(self.to_glib_none().0, size);
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer-size",
                transmute(notify_buffer_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_buffer_size_trampoline<P>(this: *mut ffi::GBufferedInputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BufferedInputStream> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&BufferedInputStream::from_glib_borrow(this).downcast_unchecked())
}
