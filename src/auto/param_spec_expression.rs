// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct ParamSpecExpression(Object<gtk_sys::GtkParamSpecExpression, ParamSpecExpressionClass>);

    match fn {
        get_type => || gtk_sys::gtk_param_expression_get_type(),
    }
}

impl ParamSpecExpression {}

impl fmt::Display for ParamSpecExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParamSpecExpression")
    }
}
