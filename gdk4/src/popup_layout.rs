use crate::PopupLayout;
use glib::translate::*;

pub trait PopupLayoutExtManual {
    fn get_offset(&self) -> (i32, i32);
}

impl PopupLayoutExtManual for PopupLayout {
    fn get_offset(&self) -> (i32, i32) {
        let mut dx = 0;
        let mut dy = 0;
        unsafe {
            ffi::gdk_popup_layout_get_offset(self.to_glib_none().0, &mut dx, &mut dy);
        }
        (dx, dy)
    }
}
