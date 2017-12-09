// This file was generated by gir (d8a605d) from gir-files (469db10)
// DO NOT EDIT

use MenuItem;
use MenuModel;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Menu(Object<ffi::GMenu>): MenuModel;

    match fn {
        get_type => || ffi::g_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        unsafe {
            from_glib_full(ffi::g_menu_new())
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MenuExt {
    fn append<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, label: P, detailed_action: Q);

    fn append_item(&self, item: &MenuItem);

    fn append_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, section: &Q);

    fn append_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, submenu: &Q);

    fn freeze(&self);

    fn insert<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, position: i32, label: P, detailed_action: Q);

    fn insert_item(&self, position: i32, item: &MenuItem);

    fn insert_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, position: i32, label: P, section: &Q);

    fn insert_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, position: i32, label: P, submenu: &Q);

    fn prepend<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, label: P, detailed_action: Q);

    fn prepend_item(&self, item: &MenuItem);

    fn prepend_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, section: &Q);

    fn prepend_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, submenu: &Q);

    fn remove(&self, position: i32);

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn remove_all(&self);
}

impl<O: IsA<Menu>> MenuExt for O {
    fn append<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, label: P, detailed_action: Q) {
        let label = label.into();
        let label = label.to_glib_none();
        let detailed_action = detailed_action.into();
        let detailed_action = detailed_action.to_glib_none();
        unsafe {
            ffi::g_menu_append(self.to_glib_none().0, label.0, detailed_action.0);
        }
    }

    fn append_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_append_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    fn append_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, section: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_append_section(self.to_glib_none().0, label.0, section.to_glib_none().0);
        }
    }

    fn append_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, submenu: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_append_submenu(self.to_glib_none().0, label.0, submenu.to_glib_none().0);
        }
    }

    fn freeze(&self) {
        unsafe {
            ffi::g_menu_freeze(self.to_glib_none().0);
        }
    }

    fn insert<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, position: i32, label: P, detailed_action: Q) {
        let label = label.into();
        let label = label.to_glib_none();
        let detailed_action = detailed_action.into();
        let detailed_action = detailed_action.to_glib_none();
        unsafe {
            ffi::g_menu_insert(self.to_glib_none().0, position, label.0, detailed_action.0);
        }
    }

    fn insert_item(&self, position: i32, item: &MenuItem) {
        unsafe {
            ffi::g_menu_insert_item(self.to_glib_none().0, position, item.to_glib_none().0);
        }
    }

    fn insert_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, position: i32, label: P, section: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_insert_section(self.to_glib_none().0, position, label.0, section.to_glib_none().0);
        }
    }

    fn insert_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, position: i32, label: P, submenu: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_insert_submenu(self.to_glib_none().0, position, label.0, submenu.to_glib_none().0);
        }
    }

    fn prepend<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, label: P, detailed_action: Q) {
        let label = label.into();
        let label = label.to_glib_none();
        let detailed_action = detailed_action.into();
        let detailed_action = detailed_action.to_glib_none();
        unsafe {
            ffi::g_menu_prepend(self.to_glib_none().0, label.0, detailed_action.0);
        }
    }

    fn prepend_item(&self, item: &MenuItem) {
        unsafe {
            ffi::g_menu_prepend_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    fn prepend_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, section: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_prepend_section(self.to_glib_none().0, label.0, section.to_glib_none().0);
        }
    }

    fn prepend_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(&self, label: P, submenu: &Q) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_prepend_submenu(self.to_glib_none().0, label.0, submenu.to_glib_none().0);
        }
    }

    fn remove(&self, position: i32) {
        unsafe {
            ffi::g_menu_remove(self.to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn remove_all(&self) {
        unsafe {
            ffi::g_menu_remove_all(self.to_glib_none().0);
        }
    }
}
