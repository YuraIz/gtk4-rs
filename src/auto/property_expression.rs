// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Expression;

glib_wrapper! {
    pub struct PropertyExpression(Object<gtk_sys::GtkPropertyExpression, PropertyExpressionClass>) @extends Expression;

    match fn {
        get_type => || gtk_sys::gtk_property_expression_get_type(),
    }
}

impl PropertyExpression {
    pub fn new<P: IsA<Expression>>(
        this_type: glib::types::Type,
        expression: Option<&P>,
        property_name: &str,
    ) -> PropertyExpression {
        assert_initialized_main_thread!();
        unsafe {
            Expression::from_glib_full(gtk_sys::gtk_property_expression_new(
                this_type.to_glib(),
                expression.map(|p| p.as_ref()).to_glib_full(),
                property_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    //pub fn new_for_pspec<P: IsA<Expression>>(expression: Option<&P>, pspec: /*Ignored*/&glib::ParamSpec) -> PropertyExpression {
    //    unsafe { TODO: call gtk_sys:gtk_property_expression_new_for_pspec() }
    //}

    pub fn get_expression(&self) -> Option<Expression> {
        unsafe {
            from_glib_none(gtk_sys::gtk_property_expression_get_expression(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn get_pspec(&self) -> /*Ignored*/Option<glib::ParamSpec> {
    //    unsafe { TODO: call gtk_sys:gtk_property_expression_get_pspec() }
    //}
}

impl fmt::Display for PropertyExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyExpression")
    }
}
