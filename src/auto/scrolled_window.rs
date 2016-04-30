// This file was generated by gir (324239f) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Bin;
use Container;
use CornerType;
use DirectionType;
use Object;
use PolicyType;
#[cfg(feature = "v3_16")]
use PositionType;
use ScrollType;
use ShadowType;
use Widget;
use ffi;
use ffi::GtkDirectionType;
#[cfg(feature = "v3_16")]
use ffi::GtkPositionType;
use ffi::GtkScrollType;
use ffi::GtkScrolledWindow;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gboolean;
use glib_ffi::gpointer;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct ScrolledWindow(Object<ffi::GtkScrolledWindow>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_scrolled_window_get_type(),
    }
}

impl ScrolledWindow {
    pub fn new(hadjustment: Option<&Adjustment>, vadjustment: Option<&Adjustment>) -> ScrolledWindow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scrolled_window_new(hadjustment.to_glib_none().0, vadjustment.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ScrolledWindowExt {
    fn add_with_viewport<T: IsA<Widget>>(&self, child: &T);

    fn get_capture_button_press(&self) -> bool;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscrollbar(&self) -> Option<Widget>;

    fn get_kinetic_scrolling(&self) -> bool;

    fn get_min_content_height(&self) -> i32;

    fn get_min_content_width(&self) -> i32;

    #[cfg(feature = "v3_16")]
    fn get_overlay_scrolling(&self) -> bool;

    fn get_placement(&self) -> CornerType;

    fn get_policy(&self) -> (PolicyType, PolicyType);

    fn get_shadow_type(&self) -> ShadowType;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscrollbar(&self) -> Option<Widget>;

    fn set_capture_button_press(&self, capture_button_press: bool);

    fn set_hadjustment(&self, hadjustment: &Adjustment);

    fn set_kinetic_scrolling(&self, kinetic_scrolling: bool);

    fn set_min_content_height(&self, height: i32);

    fn set_min_content_width(&self, width: i32);

    #[cfg(feature = "v3_16")]
    fn set_overlay_scrolling(&self, overlay_scrolling: bool);

    fn set_placement(&self, window_placement: CornerType);

    fn set_policy(&self, hscrollbar_policy: PolicyType, vscrollbar_policy: PolicyType);

    fn set_shadow_type(&self, type_: ShadowType);

    fn set_vadjustment(&self, vadjustment: &Adjustment);

    fn unset_placement(&self);

    #[cfg(feature = "v3_16")]
    fn connect_edge_overshot<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_edge_reached<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> u64;

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64;

    fn connect_scroll_child<F: Fn(&Self, ScrollType, bool) -> bool + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ScrolledWindow> + IsA<Object>> ScrolledWindowExt for O {
    fn add_with_viewport<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_scrolled_window_add_with_viewport(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn get_capture_button_press(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_capture_button_press(self.to_glib_none().0))
        }
    }

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hadjustment(self.to_glib_none().0))
        }
    }

    fn get_hscrollbar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_hscrollbar(self.to_glib_none().0))
        }
    }

    fn get_kinetic_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_kinetic_scrolling(self.to_glib_none().0))
        }
    }

    fn get_min_content_height(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_min_content_height(self.to_glib_none().0)
        }
    }

    fn get_min_content_width(&self) -> i32 {
        unsafe {
            ffi::gtk_scrolled_window_get_min_content_width(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_overlay_scrolling(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_overlay_scrolling(self.to_glib_none().0))
        }
    }

    fn get_placement(&self) -> CornerType {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_placement(self.to_glib_none().0))
        }
    }

    fn get_policy(&self) -> (PolicyType, PolicyType) {
        unsafe {
            let mut hscrollbar_policy = mem::uninitialized();
            let mut vscrollbar_policy = mem::uninitialized();
            ffi::gtk_scrolled_window_get_policy(self.to_glib_none().0, &mut hscrollbar_policy, &mut vscrollbar_policy);
            (from_glib(hscrollbar_policy), from_glib(vscrollbar_policy))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_scrolled_window_get_shadow_type(self.to_glib_none().0))
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vadjustment(self.to_glib_none().0))
        }
    }

    fn get_vscrollbar(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scrolled_window_get_vscrollbar(self.to_glib_none().0))
        }
    }

    fn set_capture_button_press(&self, capture_button_press: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_capture_button_press(self.to_glib_none().0, capture_button_press.to_glib());
        }
    }

    fn set_hadjustment(&self, hadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrolled_window_set_hadjustment(self.to_glib_none().0, hadjustment.to_glib_none().0);
        }
    }

    fn set_kinetic_scrolling(&self, kinetic_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_kinetic_scrolling(self.to_glib_none().0, kinetic_scrolling.to_glib());
        }
    }

    fn set_min_content_height(&self, height: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_height(self.to_glib_none().0, height);
        }
    }

    fn set_min_content_width(&self, width: i32) {
        unsafe {
            ffi::gtk_scrolled_window_set_min_content_width(self.to_glib_none().0, width);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_overlay_scrolling(&self, overlay_scrolling: bool) {
        unsafe {
            ffi::gtk_scrolled_window_set_overlay_scrolling(self.to_glib_none().0, overlay_scrolling.to_glib());
        }
    }

    fn set_placement(&self, window_placement: CornerType) {
        unsafe {
            ffi::gtk_scrolled_window_set_placement(self.to_glib_none().0, window_placement.to_glib());
        }
    }

    fn set_policy(&self, hscrollbar_policy: PolicyType, vscrollbar_policy: PolicyType) {
        unsafe {
            ffi::gtk_scrolled_window_set_policy(self.to_glib_none().0, hscrollbar_policy.to_glib(), vscrollbar_policy.to_glib());
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_scrolled_window_set_shadow_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_vadjustment(&self, vadjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scrolled_window_set_vadjustment(self.to_glib_none().0, vadjustment.to_glib_none().0);
        }
    }

    fn unset_placement(&self) {
        unsafe {
            ffi::gtk_scrolled_window_unset_placement(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_edge_overshot<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PositionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "edge-overshot",
                transmute(edge_overshot_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_edge_reached<F: Fn(&Self, PositionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, PositionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "edge-reached",
                transmute(edge_reached_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-focus-out",
                transmute(move_focus_out_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_scroll_child<F: Fn(&Self, ScrollType, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scroll-child",
                transmute(scroll_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn edge_overshot_trampoline<T>(this: *mut GtkScrolledWindow, pos: GtkPositionType, f: gpointer)
where T: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &Box_<Fn(&T, PositionType) + 'static> = transmute(f);
    f(&ScrolledWindow::from_glib_none(this).downcast_unchecked(), from_glib(pos))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn edge_reached_trampoline<T>(this: *mut GtkScrolledWindow, pos: GtkPositionType, f: gpointer)
where T: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &Box_<Fn(&T, PositionType) + 'static> = transmute(f);
    f(&ScrolledWindow::from_glib_none(this).downcast_unchecked(), from_glib(pos))
}

unsafe extern "C" fn move_focus_out_trampoline<T>(this: *mut GtkScrolledWindow, direction_type: GtkDirectionType, f: gpointer)
where T: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &Box_<Fn(&T, DirectionType) + 'static> = transmute(f);
    f(&ScrolledWindow::from_glib_none(this).downcast_unchecked(), from_glib(direction_type))
}

unsafe extern "C" fn scroll_child_trampoline<T>(this: *mut GtkScrolledWindow, scroll: GtkScrollType, horizontal: gboolean, f: gpointer) -> gboolean
where T: IsA<ScrolledWindow> {
    callback_guard!();
    let f: &Box_<Fn(&T, ScrollType, bool) -> bool + 'static> = transmute(f);
    f(&ScrolledWindow::from_glib_none(this).downcast_unchecked(), from_glib(scroll), from_glib(horizontal)).to_glib()
}
