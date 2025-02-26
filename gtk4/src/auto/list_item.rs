// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkListItem")]
    pub struct ListItem(Object<ffi::GtkListItem, ffi::GtkListItemClass>);

    match fn {
        type_ => || ffi::gtk_list_item_get_type(),
    }
}

impl ListItem {
    pub const NONE: Option<&'static ListItem> = None;
}

pub trait ListItemExt: 'static {
    #[doc(alias = "gtk_list_item_get_activatable")]
    #[doc(alias = "get_activatable")]
    fn is_activatable(&self) -> bool;

    #[doc(alias = "gtk_list_item_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget>;

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_item_get_focusable")]
    #[doc(alias = "get_focusable")]
    fn is_focusable(&self) -> bool;

    #[doc(alias = "gtk_list_item_get_item")]
    #[doc(alias = "get_item")]
    fn item(&self) -> Option<glib::Object>;

    #[doc(alias = "gtk_list_item_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> u32;

    #[doc(alias = "gtk_list_item_get_selectable")]
    #[doc(alias = "get_selectable")]
    fn is_selectable(&self) -> bool;

    #[doc(alias = "gtk_list_item_get_selected")]
    #[doc(alias = "get_selected")]
    fn is_selected(&self) -> bool;

    #[doc(alias = "gtk_list_item_set_activatable")]
    fn set_activatable(&self, activatable: bool);

    #[doc(alias = "gtk_list_item_set_child")]
    fn set_child(&self, child: Option<&impl IsA<Widget>>);

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_list_item_set_focusable")]
    fn set_focusable(&self, focusable: bool);

    #[doc(alias = "gtk_list_item_set_selectable")]
    fn set_selectable(&self, selectable: bool);

    #[doc(alias = "activatable")]
    fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "focusable")]
    fn connect_focusable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "item")]
    fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selectable")]
    fn connect_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selected")]
    fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListItem>> ListItemExt for O {
    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_item_get_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_child(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    fn is_focusable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_item_get_focusable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn item(&self) -> Option<glib::Object> {
        unsafe { from_glib_none(ffi::gtk_list_item_get_item(self.as_ref().to_glib_none().0)) }
    }

    fn position(&self) -> u32 {
        unsafe { ffi::gtk_list_item_get_position(self.as_ref().to_glib_none().0) }
    }

    fn is_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_item_get_selectable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_item_get_selected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_item_set_activatable(
                self.as_ref().to_glib_none().0,
                activatable.into_glib(),
            );
        }
    }

    fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_list_item_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    fn set_focusable(&self, focusable: bool) {
        unsafe {
            ffi::gtk_list_item_set_focusable(self.as_ref().to_glib_none().0, focusable.into_glib());
        }
    }

    fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_item_set_selectable(
                self.as_ref().to_glib_none().0,
                selectable.into_glib(),
            );
        }
    }

    fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<
            P: IsA<ListItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    fn connect_focusable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focusable_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focusable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focusable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_item_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_item_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selectable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<P: IsA<ListItem>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListItem")
    }
}
