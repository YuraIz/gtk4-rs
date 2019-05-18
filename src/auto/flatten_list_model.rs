// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FlattenListModel(Object<gtk_sys::GtkFlattenListModel, gtk_sys::GtkFlattenListModelClass, FlattenListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || gtk_sys::gtk_flatten_list_model_get_type(),
    }
}

impl FlattenListModel {
    pub fn new<P: IsA<gio::ListModel>>(item_type: glib::types::Type, model: Option<&P>) -> FlattenListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_flatten_list_model_new(item_type.to_glib(), model.map(|p| p.as_ref()).to_glib_none().0))
        }
    }
}

pub const NONE_FLATTEN_LIST_MODEL: Option<&FlattenListModel> = None;

pub trait FlattenListModelExt: 'static {
    fn get_model(&self) -> Option<gio::ListModel>;

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    fn get_property_item_type(&self) -> glib::types::Type;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FlattenListModel>> FlattenListModelExt for O {
    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_flatten_list_model_get_model(self.as_ref().to_glib_none().0))
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_flatten_list_model_set_model(self.as_ref().to_glib_none().0, model.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn get_property_item_type(&self) -> glib::types::Type {
        unsafe {
            let mut value = Value::from_type(<glib::types::Type as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"item-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFlattenListModel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<FlattenListModel> {
    let f: &F = &*(f as *const F);
    f(&FlattenListModel::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FlattenListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlattenListModel")
    }
}
