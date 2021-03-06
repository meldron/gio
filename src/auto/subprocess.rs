// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use Cancellable;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use InputStream;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use OutputStream;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessFlags;
use ffi;
#[cfg(feature = "futures")]
#[cfg(any(feature = "v2_40", feature = "dox"))]
use futures_core;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Subprocess(Object<ffi::GSubprocess>);

    match fn {
        get_type => || ffi::g_subprocess_get_type(),
    }
}

impl Subprocess {
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //pub fn new<'a, P: Into<Option<&'a Error>>>(flags: SubprocessFlags, error: P, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Subprocess {
    //    unsafe { TODO: call ffi::g_subprocess_new() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn newv(argv: &[&std::ffi::OsStr], flags: SubprocessFlags) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_newv(argv.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait SubprocessExt: Sized {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: Into<Option<&'b Cancellable>>>(&self, stdin_buf: P, cancellable: Q) -> Result<(Option<glib::Bytes>, Option<glib::Bytes>), Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_async<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(&self, stdin_buf: P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_async_future<'a, P: Into<Option<&'a glib::Bytes>>>(&self, stdin_buf: P) -> Box_<futures_core::Future<Item = (Self, (glib::Bytes, glib::Bytes)), Error = (Self, Error)>>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>>(&self, stdin_buf: P, cancellable: Q) -> Result<(Option<String>, Option<String>), Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn force_exit(&self);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_exit_status(&self) -> i32;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_identifier(&self) -> Option<String>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_if_exited(&self) -> bool;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_if_signaled(&self) -> bool;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_status(&self) -> i32;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stderr_pipe(&self) -> Option<InputStream>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stdin_pipe(&self) -> Option<OutputStream>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stdout_pipe(&self) -> Option<InputStream>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_successful(&self) -> bool;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_term_sig(&self) -> i32;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn send_signal(&self, signal_num: i32);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn connect_property_argv_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Subprocess> + IsA<glib::object::Object> + Clone + 'static> SubprocessExt for O {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: Into<Option<&'b Cancellable>>>(&self, stdin_buf: P, cancellable: Q) -> Result<(Option<glib::Bytes>, Option<glib::Bytes>), Error> {
        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate(self.to_glib_none().0, stdin_buf.0, cancellable.0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_async<'a, 'b, P: Into<Option<&'a glib::Bytes>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(&self, stdin_buf: P, cancellable: Q, callback: R) {
        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn communicate_async_trampoline<R: FnOnce(Result<(glib::Bytes, glib::Bytes), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_finish(_source_object as *mut _, res, &mut stdout_buf, &mut stderr_buf, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = communicate_async_trampoline::<R>;
        unsafe {
            ffi::g_subprocess_communicate_async(self.to_glib_none().0, stdin_buf.0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_async_future<'a, P: Into<Option<&'a glib::Bytes>>>(&self, stdin_buf: P) -> Box_<futures_core::Future<Item = (Self, (glib::Bytes, glib::Bytes)), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.communicate_async(
                 stdin_buf.as_ref().map(::std::borrow::Borrow::borrow),
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

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn communicate_utf8<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Cancellable>>>(&self, stdin_buf: P, cancellable: Q) -> Result<(Option<String>, Option<String>), Error> {
        let stdin_buf = stdin_buf.into();
        let stdin_buf = stdin_buf.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut stdout_buf = ptr::null_mut();
            let mut stderr_buf = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_communicate_utf8(self.to_glib_none().0, stdin_buf.0, cancellable.0, &mut stdout_buf, &mut stderr_buf, &mut error);
            if error.is_null() { Ok((from_glib_full(stdout_buf), from_glib_full(stderr_buf))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn force_exit(&self) {
        unsafe {
            ffi::g_subprocess_force_exit(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_exit_status(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_exit_status(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_identifier(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_identifier(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_if_exited(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_if_exited(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_if_signaled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_if_signaled(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_status(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_status(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stderr_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stderr_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stdin_pipe(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stdin_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_stdout_pipe(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(ffi::g_subprocess_get_stdout_pipe(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_successful(&self) -> bool {
        unsafe {
            from_glib(ffi::g_subprocess_get_successful(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_term_sig(&self) -> i32 {
        unsafe {
            ffi::g_subprocess_get_term_sig(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn send_signal(&self, signal_num: i32) {
        unsafe {
            ffi::g_subprocess_send_signal(self.to_glib_none().0, signal_num);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn wait_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_async_trampoline::<Q>;
        unsafe {
            ffi::g_subprocess_wait_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.wait_async(
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

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn wait_check_async_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let _ = ffi::g_subprocess_wait_check_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = wait_check_async_trampoline::<Q>;
        unsafe {
            ffi::g_subprocess_wait_check_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn wait_check_async_future(&self) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.wait_check_async(
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

    fn connect_property_argv_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::argv",
                transmute(notify_argv_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_argv_trampoline<P>(this: *mut ffi::GSubprocess, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Subprocess> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Subprocess::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::GSubprocess, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Subprocess> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Subprocess::from_glib_borrow(this).downcast_unchecked())
}
