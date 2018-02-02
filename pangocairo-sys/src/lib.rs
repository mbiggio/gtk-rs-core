// This file was generated by gir (f8eb742) from gir-files (77d1f70)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib;
extern crate pango_sys as pango;
extern crate cairo_sys as cairo;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

// Callbacks
pub type PangoCairoShapeRendererFunc = Option<unsafe extern "C" fn(*mut cairo::cairo_t, *mut pango::PangoAttrShape, gboolean, gpointer)>;

// Classes
#[repr(C)]
pub struct PangoCairoFcFontMap(c_void);

impl ::std::fmt::Debug for PangoCairoFcFontMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PangoCairoFcFontMap @ {:?}", self as *const _))
         .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct PangoCairoFont(c_void);

impl ::std::fmt::Debug for PangoCairoFont {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PangoCairoFont @ {:?}", self as *const _)
    }
}

#[repr(C)]
pub struct PangoCairoFontMap(c_void);

impl ::std::fmt::Debug for PangoCairoFontMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PangoCairoFontMap @ {:?}", self as *const _)
    }
}


extern "C" {

    //=========================================================================
    // PangoCairoFcFontMap
    //=========================================================================
    pub fn pango_cairo_fc_font_map_get_type() -> GType;

    //=========================================================================
    // PangoCairoFont
    //=========================================================================
    pub fn pango_cairo_font_get_type() -> GType;
    pub fn pango_cairo_font_get_scaled_font(font: *mut PangoCairoFont) -> *mut cairo::cairo_scaled_font_t;

    //=========================================================================
    // PangoCairoFontMap
    //=========================================================================
    pub fn pango_cairo_font_map_get_type() -> GType;
    pub fn pango_cairo_font_map_get_default() -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_new() -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_new_for_font_type(fonttype: cairo::enums::FontType) -> *mut pango::PangoFontMap;
    pub fn pango_cairo_font_map_create_context(fontmap: *mut PangoCairoFontMap) -> *mut pango::PangoContext;
    pub fn pango_cairo_font_map_get_font_type(fontmap: *mut PangoCairoFontMap) -> cairo::enums::FontType;
    pub fn pango_cairo_font_map_get_resolution(fontmap: *mut PangoCairoFontMap) -> c_double;
    pub fn pango_cairo_font_map_set_default(fontmap: *mut PangoCairoFontMap);
    pub fn pango_cairo_font_map_set_resolution(fontmap: *mut PangoCairoFontMap, dpi: c_double);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn pango_cairo_context_get_font_options(context: *mut pango::PangoContext) -> *const cairo::cairo_font_options_t;
    pub fn pango_cairo_context_get_resolution(context: *mut pango::PangoContext) -> c_double;
    pub fn pango_cairo_context_get_shape_renderer(context: *mut pango::PangoContext, data: *mut gpointer) -> PangoCairoShapeRendererFunc;
    pub fn pango_cairo_context_set_font_options(context: *mut pango::PangoContext, options: *const cairo::cairo_font_options_t);
    pub fn pango_cairo_context_set_resolution(context: *mut pango::PangoContext, dpi: c_double);
    pub fn pango_cairo_context_set_shape_renderer(context: *mut pango::PangoContext, func: PangoCairoShapeRendererFunc, data: gpointer, dnotify: glib::GDestroyNotify);
    pub fn pango_cairo_create_context(cr: *mut cairo::cairo_t) -> *mut pango::PangoContext;
    pub fn pango_cairo_create_layout(cr: *mut cairo::cairo_t) -> *mut pango::PangoLayout;
    pub fn pango_cairo_error_underline_path(cr: *mut cairo::cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);
    pub fn pango_cairo_glyph_string_path(cr: *mut cairo::cairo_t, font: *mut pango::PangoFont, glyphs: *mut pango::PangoGlyphString);
    pub fn pango_cairo_layout_line_path(cr: *mut cairo::cairo_t, line: *mut pango::PangoLayoutLine);
    pub fn pango_cairo_layout_path(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);
    pub fn pango_cairo_show_error_underline(cr: *mut cairo::cairo_t, x: c_double, y: c_double, width: c_double, height: c_double);
    pub fn pango_cairo_show_glyph_item(cr: *mut cairo::cairo_t, text: *const c_char, glyph_item: *mut pango::PangoGlyphItem);
    pub fn pango_cairo_show_glyph_string(cr: *mut cairo::cairo_t, font: *mut pango::PangoFont, glyphs: *mut pango::PangoGlyphString);
    pub fn pango_cairo_show_layout(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);
    pub fn pango_cairo_show_layout_line(cr: *mut cairo::cairo_t, line: *mut pango::PangoLayoutLine);
    pub fn pango_cairo_update_context(cr: *mut cairo::cairo_t, context: *mut pango::PangoContext);
    pub fn pango_cairo_update_layout(cr: *mut cairo::cairo_t, layout: *mut pango::PangoLayout);

}
