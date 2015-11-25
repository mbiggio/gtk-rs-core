// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A "light switch" style toggle.

use glib::translate::*;
use ffi;

use glib::object::Downcast;
use super::widget::Widget;

/// A "light switch" style toggle.
glib_wrapper! {
    pub struct Switch(Object<ffi::GtkSwitch>): Widget, ::Actionable, ::Buildable;

    match fn {
        get_type => || ffi::gtk_switch_get_type(),
    }
}

impl Switch {
    pub fn new() -> Switch {
        unsafe { Widget::from_glib_none(ffi::gtk_switch_new()).downcast_unchecked() }
    }

    pub fn set_active(&self, is_active: bool) {
        unsafe { ffi::gtk_switch_set_active(self.to_glib_none().0, is_active.to_glib()); }
    }

    pub fn get_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_switch_get_active(self.to_glib_none().0)) }
    }
}
