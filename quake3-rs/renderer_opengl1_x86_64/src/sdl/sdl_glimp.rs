use ::libc;

pub mod sdl_icon_h {
    /* GIMP RGBA C-Source image dump (sdl_icon.h) */

    pub static mut CLIENT_WINDOW_ICON: crate::sdl_icon_h::C2RustUnnamed_152 = {
        let mut init = crate::sdl_icon_h::C2RustUnnamed_152 {
            width: 32u32,
            height: 32u32,
            bytes_per_pixel: 4u32,
            pixel_data: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119,
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255,
                136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136,
                0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 136, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                170, 0, 0, 255, 0, 0, 0, 0, 187, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 187,
                0, 0, 255, 204, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0,
                255, 0, 0, 0, 0, 221, 0, 0, 255, 204, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0, 255, 221, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 204, 0, 0, 255, 221, 0, 0, 255, 221, 0,
                0, 255, 187, 0, 0, 255, 153, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 170, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170,
                0, 0, 255, 187, 0, 0, 255, 221, 0, 0, 255, 221, 0, 0, 255, 204, 0, 0, 255, 153, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 170, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0,
                255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 0, 0, 0, 0,
                170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0,
                0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0, 255, 204, 0, 0,
                255, 170, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 255, 153, 0, 0, 255, 187, 0, 0, 255, 204, 0, 0,
                255, 204, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 204,
                0, 0, 255, 204, 0, 0, 255, 187, 0, 0, 255, 153, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0,
                0, 255, 187, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0,
                187, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0,
                255, 187, 0, 0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 170, 0, 0, 255, 0, 0, 0, 0, 187,
                0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0,
                0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0,
                0, 136, 0, 0, 255, 153, 0, 0, 255, 0, 0, 0, 0, 170, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255,
                136, 0, 0, 255, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 136, 0, 0, 255, 0,
                0, 0, 0, 153, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0,
                0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0,
                255, 0, 0, 0, 0, 119, 0, 0, 255, 136, 0, 0, 255, 0, 0, 0, 0, 136, 0, 0, 255, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0,
                119, 0, 0, 255, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255,
                119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 119, 0, 0, 255, 119, 0, 0, 255, 0,
                0, 0, 0, 119, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 102, 0, 0, 255, 102, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                102, 0, 0, 255, 102, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };
        init
    };
}

pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::stddef_h::ptrdiff_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::SDL_Color;
pub use crate::stdlib::SDL_Palette;
pub use crate::stdlib::SDL_PixelFormat;
pub use crate::stdlib::SDL_Rect;
pub use crate::stdlib::SDL_calloc;
pub use crate::stdlib::SDL_free;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::SDL_bool;
pub use crate::stdlib::Uint32;
pub use crate::stdlib::Uint8;
pub use crate::stdlib::SDL_FALSE;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR1555;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR32;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR4444;
pub use crate::stdlib::SDL_PIXELFORMAT_ABGR8888;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB1555;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB2101010;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB32;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB4444;
pub use crate::stdlib::SDL_PIXELFORMAT_ARGB8888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR24;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR555;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR565;
pub use crate::stdlib::SDL_PIXELFORMAT_BGR888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA32;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA4444;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA5551;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRA8888;
pub use crate::stdlib::SDL_PIXELFORMAT_BGRX8888;
pub use crate::stdlib::SDL_PIXELFORMAT_EXTERNAL_OES;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX1LSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX1MSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX4LSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX4MSB;
pub use crate::stdlib::SDL_PIXELFORMAT_INDEX8;
pub use crate::stdlib::SDL_PIXELFORMAT_IYUV;
pub use crate::stdlib::SDL_PIXELFORMAT_NV12;
pub use crate::stdlib::SDL_PIXELFORMAT_NV21;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB24;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB332;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB444;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB555;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB565;
pub use crate::stdlib::SDL_PIXELFORMAT_RGB888;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA32;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA4444;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA5551;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBA8888;
pub use crate::stdlib::SDL_PIXELFORMAT_RGBX8888;
pub use crate::stdlib::SDL_PIXELFORMAT_UNKNOWN;
pub use crate::stdlib::SDL_PIXELFORMAT_UYVY;
pub use crate::stdlib::SDL_PIXELFORMAT_YUY2;
pub use crate::stdlib::SDL_PIXELFORMAT_YV12;
pub use crate::stdlib::SDL_PIXELFORMAT_YVYU;
pub use crate::stdlib::SDL_TRUE;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::qcommon::q_shared::PRINT_ALL;
pub use crate::src::qcommon::q_shared::PRINT_DEVELOPER;
pub use crate::src::qcommon::q_shared::PRINT_ERROR;
pub use crate::src::qcommon::q_shared::PRINT_WARNING;
pub use crate::src::renderergl1::tr_subs::Com_Error;
pub use crate::stdlib::SDL_BlitMap;
pub use crate::stdlib::SDL_CreateRGBSurfaceFrom;
pub use crate::stdlib::SDL_CreateWindow;
pub use crate::stdlib::SDL_DestroyWindow;
pub use crate::stdlib::SDL_DisplayMode;
pub use crate::stdlib::SDL_FreeSurface;
pub use crate::stdlib::SDL_GLContext;
pub use crate::stdlib::SDL_GL_CreateContext;
pub use crate::stdlib::SDL_GL_DeleteContext;
pub use crate::stdlib::SDL_GL_ExtensionSupported;
pub use crate::stdlib::SDL_GL_GetAttribute;
pub use crate::stdlib::SDL_GL_GetProcAddress;
pub use crate::stdlib::SDL_GL_SetAttribute;
pub use crate::stdlib::SDL_GL_SetSwapInterval;
pub use crate::stdlib::SDL_GL_SwapWindow;
pub use crate::stdlib::SDL_GLattr;
pub use crate::stdlib::SDL_GetCurrentVideoDriver;
pub use crate::stdlib::SDL_GetDesktopDisplayMode;
pub use crate::stdlib::SDL_GetDisplayMode;
pub use crate::stdlib::SDL_GetNumDisplayModes;
pub use crate::stdlib::SDL_GetWindowDisplayIndex;
pub use crate::stdlib::SDL_GetWindowDisplayMode;
pub use crate::stdlib::SDL_GetWindowFlags;
pub use crate::stdlib::SDL_GetWindowPosition;
pub use crate::stdlib::SDL_MinimizeWindow;
pub use crate::stdlib::SDL_SetWindowBrightness;
pub use crate::stdlib::SDL_SetWindowDisplayMode;
pub use crate::stdlib::SDL_SetWindowFullscreen;
pub use crate::stdlib::SDL_SetWindowIcon;
pub use crate::stdlib::SDL_Surface;
pub use crate::stdlib::SDL_Window;
pub use crate::stdlib::SDL_GL_ACCELERATED_VISUAL;
pub use crate::stdlib::SDL_GL_ACCUM_ALPHA_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_BLUE_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_GREEN_SIZE;
pub use crate::stdlib::SDL_GL_ACCUM_RED_SIZE;
pub use crate::stdlib::SDL_GL_ALPHA_SIZE;
pub use crate::stdlib::SDL_GL_BLUE_SIZE;
pub use crate::stdlib::SDL_GL_BUFFER_SIZE;
pub use crate::stdlib::SDL_GL_CONTEXT_EGL;
pub use crate::stdlib::SDL_GL_CONTEXT_FLAGS;
pub use crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION;
pub use crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION;
pub use crate::stdlib::SDL_GL_CONTEXT_NO_ERROR;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_COMPATIBILITY;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_CORE;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_ES;
pub use crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK;
pub use crate::stdlib::SDL_GL_CONTEXT_RELEASE_BEHAVIOR;
pub use crate::stdlib::SDL_GL_CONTEXT_RESET_NOTIFICATION;
pub use crate::stdlib::SDL_GL_DEPTH_SIZE;
pub use crate::stdlib::SDL_GL_DOUBLEBUFFER;
pub use crate::stdlib::SDL_GL_FRAMEBUFFER_SRGB_CAPABLE;
pub use crate::stdlib::SDL_GL_GREEN_SIZE;
pub use crate::stdlib::SDL_GL_MULTISAMPLEBUFFERS;
pub use crate::stdlib::SDL_GL_MULTISAMPLESAMPLES;
pub use crate::stdlib::SDL_GL_RED_SIZE;
pub use crate::stdlib::SDL_GL_RETAINED_BACKING;
pub use crate::stdlib::SDL_GL_SHARE_WITH_CURRENT_CONTEXT;
pub use crate::stdlib::SDL_GL_STENCIL_SIZE;
pub use crate::stdlib::SDL_GL_STEREO;
pub use crate::stdlib::SDL_WINDOW_ALLOW_HIGHDPI;
pub use crate::stdlib::SDL_WINDOW_ALWAYS_ON_TOP;
pub use crate::stdlib::SDL_WINDOW_BORDERLESS;
pub use crate::stdlib::SDL_WINDOW_FOREIGN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN;
pub use crate::stdlib::SDL_WINDOW_FULLSCREEN_DESKTOP;
pub use crate::stdlib::SDL_WINDOW_HIDDEN;
pub use crate::stdlib::SDL_WINDOW_INPUT_FOCUS;
pub use crate::stdlib::SDL_WINDOW_INPUT_GRABBED;
pub use crate::stdlib::SDL_WINDOW_MAXIMIZED;
pub use crate::stdlib::SDL_WINDOW_MINIMIZED;
pub use crate::stdlib::SDL_WINDOW_MOUSE_CAPTURE;
pub use crate::stdlib::SDL_WINDOW_MOUSE_FOCUS;
pub use crate::stdlib::SDL_WINDOW_OPENGL;
pub use crate::stdlib::SDL_WINDOW_POPUP_MENU;
pub use crate::stdlib::SDL_WINDOW_RESIZABLE;
pub use crate::stdlib::SDL_WINDOW_SHOWN;
pub use crate::stdlib::SDL_WINDOW_SKIP_TASKBAR;
pub use crate::stdlib::SDL_WINDOW_TOOLTIP;
pub use crate::stdlib::SDL_WINDOW_UTILITY;
pub use crate::stdlib::SDL_WINDOW_VULKAN;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::qgl_h::ActiveTextureproc;
pub use crate::qgl_h::AlphaFuncproc;
pub use crate::qgl_h::ArrayElementproc;
pub use crate::qgl_h::AttachShaderproc;
pub use crate::qgl_h::BeginQueryproc;
pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::BindAttribLocationproc;
pub use crate::qgl_h::BindBufferproc;
pub use crate::qgl_h::BindFramebufferproc;
pub use crate::qgl_h::BindMultiTextureEXTproc;
pub use crate::qgl_h::BindRenderbufferproc;
pub use crate::qgl_h::BindTextureproc;
pub use crate::qgl_h::BindVertexArrayproc;
pub use crate::qgl_h::BlendFuncproc;
pub use crate::qgl_h::BlitFramebufferproc;
pub use crate::qgl_h::BufferDataproc;
pub use crate::qgl_h::BufferSubDataproc;
pub use crate::qgl_h::CheckFramebufferStatusproc;
pub use crate::qgl_h::CheckNamedFramebufferStatusEXTproc;
pub use crate::qgl_h::ClearColorproc;
pub use crate::qgl_h::ClearDepthfproc;
pub use crate::qgl_h::ClearDepthproc;
pub use crate::qgl_h::ClearStencilproc;
pub use crate::qgl_h::Clearproc;
pub use crate::qgl_h::ClipPlanefproc;
pub use crate::qgl_h::ClipPlaneproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::Color4fproc;
pub use crate::qgl_h::Color4ubvproc;
pub use crate::qgl_h::ColorMaskproc;
pub use crate::qgl_h::ColorPointerproc;
pub use crate::qgl_h::CompileShaderproc;
pub use crate::qgl_h::CompressedTexImage2Dproc;
pub use crate::qgl_h::CompressedTexSubImage2Dproc;
pub use crate::qgl_h::CompressedTextureImage2DEXTproc;
pub use crate::qgl_h::CompressedTextureSubImage2DEXTproc;
pub use crate::qgl_h::CopyTexSubImage2Dproc;
pub use crate::qgl_h::CopyTextureSubImage2DEXTproc;
pub use crate::qgl_h::CreateProgramproc;
pub use crate::qgl_h::CreateShaderproc;
pub use crate::qgl_h::CullFaceproc;
pub use crate::qgl_h::DeleteBuffersproc;
pub use crate::qgl_h::DeleteFramebuffersproc;
pub use crate::qgl_h::DeleteProgramproc;
pub use crate::qgl_h::DeleteQueriesproc;
pub use crate::qgl_h::DeleteRenderbuffersproc;
pub use crate::qgl_h::DeleteShaderproc;
pub use crate::qgl_h::DeleteTexturesproc;
pub use crate::qgl_h::DeleteVertexArraysproc;
pub use crate::qgl_h::DepthFuncproc;
pub use crate::qgl_h::DepthMaskproc;
pub use crate::qgl_h::DepthRangefproc;
pub use crate::qgl_h::DepthRangeproc;
pub use crate::qgl_h::DetachShaderproc;
pub use crate::qgl_h::DisableClientStateproc;
pub use crate::qgl_h::DisableVertexAttribArrayproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::DrawArraysproc;
pub use crate::qgl_h::DrawBufferproc;
pub use crate::qgl_h::DrawElementsproc;
pub use crate::qgl_h::EnableClientStateproc;
pub use crate::qgl_h::EnableVertexAttribArrayproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::EndQueryproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::Finishproc;
pub use crate::qgl_h::Flushproc;
pub use crate::qgl_h::FramebufferRenderbufferproc;
pub use crate::qgl_h::FramebufferTexture2Dproc;
pub use crate::qgl_h::Frustumfproc;
pub use crate::qgl_h::Frustumproc;
pub use crate::qgl_h::GenBuffersproc;
pub use crate::qgl_h::GenFramebuffersproc;
pub use crate::qgl_h::GenQueriesproc;
pub use crate::qgl_h::GenRenderbuffersproc;
pub use crate::qgl_h::GenTexturesproc;
pub use crate::qgl_h::GenVertexArraysproc;
pub use crate::qgl_h::GenerateMipmapproc;
pub use crate::qgl_h::GenerateTextureMipmapEXTproc;
pub use crate::qgl_h::GetActiveUniformproc;
pub use crate::qgl_h::GetBooleanvproc;
pub use crate::qgl_h::GetErrorproc;
pub use crate::qgl_h::GetIntegervproc;
pub use crate::qgl_h::GetProgramInfoLogproc;
pub use crate::qgl_h::GetProgramivproc;
pub use crate::qgl_h::GetQueryObjectivproc;
pub use crate::qgl_h::GetQueryObjectuivproc;
pub use crate::qgl_h::GetShaderInfoLogproc;
pub use crate::qgl_h::GetShaderSourceproc;
pub use crate::qgl_h::GetShaderivproc;
pub use crate::qgl_h::GetStringiproc;
pub use crate::qgl_h::GetStringproc;
pub use crate::qgl_h::GetUniformLocationproc;
pub use crate::qgl_h::LineWidthproc;
pub use crate::qgl_h::LinkProgramproc;
pub use crate::qgl_h::LoadIdentityproc;
pub use crate::qgl_h::LoadMatrixfproc;
pub use crate::qgl_h::MatrixModeproc;
pub use crate::qgl_h::NamedFramebufferRenderbufferEXTproc;
pub use crate::qgl_h::NamedFramebufferTexture2DEXTproc;
pub use crate::qgl_h::NamedRenderbufferStorageEXTproc;
pub use crate::qgl_h::NamedRenderbufferStorageMultisampleEXTproc;
pub use crate::qgl_h::Orthofproc;
pub use crate::qgl_h::Orthoproc;
pub use crate::qgl_h::PolygonModeproc;
pub use crate::qgl_h::PolygonOffsetproc;
pub use crate::qgl_h::PopMatrixproc;
pub use crate::qgl_h::ProgramUniform1fEXTproc;
pub use crate::qgl_h::ProgramUniform1fvEXTproc;
pub use crate::qgl_h::ProgramUniform1iEXTproc;
pub use crate::qgl_h::ProgramUniform2fEXTproc;
pub use crate::qgl_h::ProgramUniform3fEXTproc;
pub use crate::qgl_h::ProgramUniform4fEXTproc;
pub use crate::qgl_h::ProgramUniformMatrix4fvEXTproc;
pub use crate::qgl_h::PushMatrixproc;
pub use crate::qgl_h::ReadPixelsproc;
pub use crate::qgl_h::RenderbufferStorageMultisampleproc;
pub use crate::qgl_h::RenderbufferStorageproc;
pub use crate::qgl_h::Scissorproc;
pub use crate::qgl_h::ShadeModelproc;
pub use crate::qgl_h::ShaderSourceproc;
pub use crate::qgl_h::StencilFuncproc;
pub use crate::qgl_h::StencilMaskproc;
pub use crate::qgl_h::StencilOpproc;
pub use crate::qgl_h::TexCoord2fproc;
pub use crate::qgl_h::TexCoord2fvproc;
pub use crate::qgl_h::TexCoordPointerproc;
pub use crate::qgl_h::TexEnvfproc;
pub use crate::qgl_h::TexImage2Dproc;
pub use crate::qgl_h::TexParameterfproc;
pub use crate::qgl_h::TexParameteriproc;
pub use crate::qgl_h::TexSubImage2Dproc;
pub use crate::qgl_h::TextureImage2DEXTproc;
pub use crate::qgl_h::TextureParameterfEXTproc;
pub use crate::qgl_h::TextureParameteriEXTproc;
pub use crate::qgl_h::TextureSubImage2DEXTproc;
pub use crate::qgl_h::Translatefproc;
pub use crate::qgl_h::Uniform1fproc;
pub use crate::qgl_h::Uniform1fvproc;
pub use crate::qgl_h::Uniform1iproc;
pub use crate::qgl_h::Uniform2fproc;
pub use crate::qgl_h::Uniform3fproc;
pub use crate::qgl_h::Uniform4fproc;
pub use crate::qgl_h::UniformMatrix4fvproc;
pub use crate::qgl_h::UseProgramproc;
pub use crate::qgl_h::ValidateProgramproc;
pub use crate::qgl_h::Vertex2fproc;
pub use crate::qgl_h::Vertex3fproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::qgl_h::VertexAttribPointerproc;
pub use crate::qgl_h::VertexPointerproc;
pub use crate::qgl_h::Viewportproc;
pub use crate::sdl_icon_h::C2RustUnnamed_152;
pub use crate::src::sdl::sdl_glimp::sdl_icon_h::CLIENT_WINDOW_ICON;
use crate::stdlib::fabs;
use crate::stdlib::memset;
use crate::stdlib::sscanf;
use crate::stdlib::strlen;
use crate::stdlib::strstr;
pub use crate::stdlib::GLbitfield;
pub use crate::stdlib::GLboolean;
pub use crate::stdlib::GLchar;
pub use crate::stdlib::GLclampd;
pub use crate::stdlib::GLclampf;
pub use crate::stdlib::GLdouble;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLintptr;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLsizeiptr;
pub use crate::stdlib::GLubyte;
pub use crate::stdlib::GLuint;
pub use crate::stdlib::GLvoid;
use crate::stdlib::SDL_GetError;

use crate::src::renderergl1::tr_init::displayAspect;
use crate::src::renderergl1::tr_init::glConfig;
use crate::src::renderergl1::tr_init::maxAnisotropy;
use crate::src::renderergl1::tr_init::r_allowExtensions;
use crate::src::renderergl1::tr_init::r_colorbits;
use crate::src::renderergl1::tr_init::r_depthbits;
use crate::src::renderergl1::tr_init::r_drawBuffer;
use crate::src::renderergl1::tr_init::r_ext_compiled_vertex_array;
use crate::src::renderergl1::tr_init::r_ext_compressed_textures;
use crate::src::renderergl1::tr_init::r_ext_multisample;
use crate::src::renderergl1::tr_init::r_ext_multitexture;
use crate::src::renderergl1::tr_init::r_ext_texture_env_add;
use crate::src::renderergl1::tr_init::r_ext_texture_filter_anisotropic;
use crate::src::renderergl1::tr_init::r_fullscreen;
use crate::src::renderergl1::tr_init::r_ignorehwgamma;
use crate::src::renderergl1::tr_init::r_mode;
use crate::src::renderergl1::tr_init::r_noborder;
use crate::src::renderergl1::tr_init::r_stencilbits;
use crate::src::renderergl1::tr_init::r_stereoEnabled;
use crate::src::renderergl1::tr_init::r_swapInterval;
use crate::src::renderergl1::tr_init::textureFilterAnisotropic;
use crate::src::renderergl1::tr_init::R_GetModeInfo;
use crate::src::renderergl1::tr_main::ri;
use crate::stdlib::SDL_Init;
use crate::stdlib::SDL_QuitSubSystem;
use crate::stdlib::SDL_WasInit;

pub const RSERR_INVALID_MODE: rserr_t = 2;

pub const RSERR_INVALID_FULLSCREEN: rserr_t = 1;

pub type rserr_t = u32;

pub const RSERR_UNKNOWN: rserr_t = 3;

pub const RSERR_OK: rserr_t = 0;
#[no_mangle]

pub static mut SDL_window: *mut crate::stdlib::SDL_Window =
    
    0 as *mut crate::stdlib::SDL_Window;

static mut SDL_glContext: crate::stdlib::SDL_GLContext =
    
    0 as *mut libc::c_void;
#[no_mangle]

pub static mut r_allowSoftwareGL: *mut crate::src::qcommon::q_shared::cvar_t =
    
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
// Don't abort out if a hardware visual can't be obtained
#[no_mangle]

pub static mut r_allowResize: *mut crate::src::qcommon::q_shared::cvar_t =
    
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
// make window resizable
#[no_mangle]

pub static mut r_centerWindow: *mut crate::src::qcommon::q_shared::cvar_t =
    
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_sdlDriver: *mut crate::src::qcommon::q_shared::cvar_t =
    
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut qglMajorVersion: i32 = 0;
#[no_mangle]

pub static mut qglMinorVersion: i32 = 0;
#[no_mangle]

pub static mut qglesMajorVersion: i32 = 0;
#[no_mangle]

pub static mut qglesMinorVersion: i32 = 0;
#[no_mangle]

pub static mut qglActiveTextureARB: Option<unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ()> =
    None;
#[no_mangle]

pub static mut qglClientActiveTextureARB: Option<
    unsafe extern "C" fn(_: crate::stdlib::GLenum) -> (),
> = None;
#[no_mangle]

pub static mut qglMultiTexCoord2fARB: Option<
    unsafe extern "C" fn(
        _: crate::stdlib::GLenum,
        _: crate::stdlib::GLfloat,
        _: crate::stdlib::GLfloat,
    ) -> (),
> = None;
#[no_mangle]

pub static mut qglLockArraysEXT: Option<
    unsafe extern "C" fn(_: crate::stdlib::GLint, _: crate::stdlib::GLsizei) -> (),
> = None;
#[no_mangle]

pub static mut qglUnlockArraysEXT: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]

pub static mut qglGenTextures: Option<crate::qgl_h::GenTexturesproc> = None;
#[no_mangle]

pub static mut qglBindTexture: Option<crate::qgl_h::BindTextureproc> = None;
#[no_mangle]

pub static mut qglBlendFunc: Option<crate::qgl_h::BlendFuncproc> = None;
#[no_mangle]

pub static mut qglClearStencil: Option<crate::qgl_h::ClearStencilproc> = None;
#[no_mangle]

pub static mut qglColorMask: Option<crate::qgl_h::ColorMaskproc> = None;
#[no_mangle]

pub static mut qglCopyTexSubImage2D: Option<crate::qgl_h::CopyTexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglCullFace: Option<crate::qgl_h::CullFaceproc> = None;
#[no_mangle]

pub static mut qglDeleteTextures: Option<crate::qgl_h::DeleteTexturesproc> = None;
#[no_mangle]

pub static mut qglDepthFunc: Option<crate::qgl_h::DepthFuncproc> = None;
#[no_mangle]

pub static mut qglDepthMask: Option<crate::qgl_h::DepthMaskproc> = None;
#[no_mangle]

pub static mut qglDisable: Option<crate::qgl_h::Disableproc> = None;
#[no_mangle]

pub static mut qglDrawArrays: Option<crate::qgl_h::DrawArraysproc> = None;
#[no_mangle]

pub static mut qglDrawElements: Option<crate::qgl_h::DrawElementsproc> = None;
#[no_mangle]

pub static mut qglEnable: Option<crate::qgl_h::Enableproc> = None;
#[no_mangle]

pub static mut qglFinish: Option<crate::qgl_h::Finishproc> = None;
#[no_mangle]

pub static mut qglFlush: Option<crate::qgl_h::Flushproc> = None;
#[no_mangle]

pub static mut qglGetBooleanv: Option<crate::qgl_h::GetBooleanvproc> = None;
#[no_mangle]

pub static mut qglGetError: Option<crate::qgl_h::GetErrorproc> = None;
#[no_mangle]

pub static mut qglLineWidth: Option<crate::qgl_h::LineWidthproc> = None;
#[no_mangle]

pub static mut qglPolygonOffset: Option<crate::qgl_h::PolygonOffsetproc> = None;
#[no_mangle]

pub static mut qglReadPixels: Option<crate::qgl_h::ReadPixelsproc> = None;
#[no_mangle]

pub static mut qglScissor: Option<crate::qgl_h::Scissorproc> = None;
#[no_mangle]

pub static mut qglStencilFunc: Option<crate::qgl_h::StencilFuncproc> = None;
#[no_mangle]

pub static mut qglClearColor: Option<crate::qgl_h::ClearColorproc> = None;
#[no_mangle]

pub static mut qglClear: Option<crate::qgl_h::Clearproc> = None;
#[no_mangle]

pub static mut qglStencilMask: Option<crate::qgl_h::StencilMaskproc> = None;
#[no_mangle]

pub static mut qglStencilOp: Option<crate::qgl_h::StencilOpproc> = None;
#[no_mangle]

pub static mut qglTexImage2D: Option<crate::qgl_h::TexImage2Dproc> = None;
#[no_mangle]

pub static mut qglTexParameterf: Option<crate::qgl_h::TexParameterfproc> = None;
#[no_mangle]

pub static mut qglTexParameteri: Option<crate::qgl_h::TexParameteriproc> = None;
#[no_mangle]

pub static mut qglTexSubImage2D: Option<crate::qgl_h::TexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglGetString: Option<crate::qgl_h::GetStringproc> = None;
#[no_mangle]

pub static mut qglGetIntegerv: Option<crate::qgl_h::GetIntegervproc> = None;
#[no_mangle]

pub static mut qglTranslatef: Option<crate::qgl_h::Translatefproc> = None;
#[no_mangle]

pub static mut qglViewport: Option<crate::qgl_h::Viewportproc> = None;
#[no_mangle]

pub static mut qglAlphaFunc: Option<crate::qgl_h::AlphaFuncproc> = None;
#[no_mangle]

pub static mut qglColor4f: Option<crate::qgl_h::Color4fproc> = None;
#[no_mangle]

pub static mut qglColorPointer: Option<crate::qgl_h::ColorPointerproc> = None;
#[no_mangle]

pub static mut qglDisableClientState: Option<crate::qgl_h::DisableClientStateproc> = None;
#[no_mangle]

pub static mut qglEnableClientState: Option<crate::qgl_h::EnableClientStateproc> = None;
#[no_mangle]

pub static mut qglLoadIdentity: Option<crate::qgl_h::LoadIdentityproc> = None;
#[no_mangle]

pub static mut qglLoadMatrixf: Option<crate::qgl_h::LoadMatrixfproc> = None;
#[no_mangle]

pub static mut qglMatrixMode: Option<crate::qgl_h::MatrixModeproc> = None;
#[no_mangle]

pub static mut qglPopMatrix: Option<crate::qgl_h::PopMatrixproc> = None;
#[no_mangle]

pub static mut qglPushMatrix: Option<crate::qgl_h::PushMatrixproc> = None;
#[no_mangle]

pub static mut qglShadeModel: Option<crate::qgl_h::ShadeModelproc> = None;
#[no_mangle]

pub static mut qglTexCoordPointer: Option<crate::qgl_h::TexCoordPointerproc> = None;
#[no_mangle]

pub static mut qglTexEnvf: Option<crate::qgl_h::TexEnvfproc> = None;
#[no_mangle]

pub static mut qglVertexPointer: Option<crate::qgl_h::VertexPointerproc> = None;
#[no_mangle]

pub static mut qglClearDepth: Option<crate::qgl_h::ClearDepthproc> = None;
#[no_mangle]

pub static mut qglDepthRange: Option<crate::qgl_h::DepthRangeproc> = None;
#[no_mangle]

pub static mut qglDrawBuffer: Option<crate::qgl_h::DrawBufferproc> = None;
#[no_mangle]

pub static mut qglPolygonMode: Option<crate::qgl_h::PolygonModeproc> = None;
#[no_mangle]

pub static mut qglTexCoord2f: Option<crate::qgl_h::TexCoord2fproc> = None;
#[no_mangle]

pub static mut qglArrayElement: Option<crate::qgl_h::ArrayElementproc> = None;
#[no_mangle]

pub static mut qglBegin: Option<crate::qgl_h::Beginproc> = None;
#[no_mangle]

pub static mut qglClipPlane: Option<crate::qgl_h::ClipPlaneproc> = None;
#[no_mangle]

pub static mut qglColor3f: Option<crate::qgl_h::Color3fproc> = None;
#[no_mangle]

pub static mut qglColor4ubv: Option<crate::qgl_h::Color4ubvproc> = None;
#[no_mangle]

pub static mut qglEnd: Option<crate::qgl_h::Endproc> = None;
#[no_mangle]

pub static mut qglFrustum: Option<crate::qgl_h::Frustumproc> = None;
#[no_mangle]

pub static mut qglOrtho: Option<crate::qgl_h::Orthoproc> = None;
#[no_mangle]

pub static mut qglTexCoord2fv: Option<crate::qgl_h::TexCoord2fvproc> = None;
#[no_mangle]

pub static mut qglVertex2f: Option<crate::qgl_h::Vertex2fproc> = None;
#[no_mangle]

pub static mut qglVertex3f: Option<crate::qgl_h::Vertex3fproc> = None;
#[no_mangle]

pub static mut qglVertex3fv: Option<crate::qgl_h::Vertex3fvproc> = None;
#[no_mangle]

pub static mut qglClearDepthf: Option<crate::qgl_h::ClearDepthfproc> = None;
#[no_mangle]

pub static mut qglDepthRangef: Option<crate::qgl_h::DepthRangefproc> = None;
#[no_mangle]

pub static mut qglClipPlanef: Option<crate::qgl_h::ClipPlanefproc> = None;
#[no_mangle]

pub static mut qglFrustumf: Option<crate::qgl_h::Frustumfproc> = None;
#[no_mangle]

pub static mut qglOrthof: Option<crate::qgl_h::Orthofproc> = None;
#[no_mangle]

pub static mut qglActiveTexture: Option<crate::qgl_h::ActiveTextureproc> = None;
#[no_mangle]

pub static mut qglCompressedTexImage2D: Option<crate::qgl_h::CompressedTexImage2Dproc> = None;
#[no_mangle]

pub static mut qglCompressedTexSubImage2D: Option<crate::qgl_h::CompressedTexSubImage2Dproc> = None;
#[no_mangle]

pub static mut qglBufferSubData: Option<crate::qgl_h::BufferSubDataproc> = None;
#[no_mangle]

pub static mut qglBindBuffer: Option<crate::qgl_h::BindBufferproc> = None;
#[no_mangle]

pub static mut qglDeleteBuffers: Option<crate::qgl_h::DeleteBuffersproc> = None;
#[no_mangle]

pub static mut qglGenBuffers: Option<crate::qgl_h::GenBuffersproc> = None;
#[no_mangle]

pub static mut qglBufferData: Option<crate::qgl_h::BufferDataproc> = None;
#[no_mangle]

pub static mut qglCompileShader: Option<crate::qgl_h::CompileShaderproc> = None;
#[no_mangle]

pub static mut qglGetShaderSource: Option<crate::qgl_h::GetShaderSourceproc> = None;
#[no_mangle]

pub static mut qglCreateShader: Option<crate::qgl_h::CreateShaderproc> = None;
#[no_mangle]

pub static mut qglCreateProgram: Option<crate::qgl_h::CreateProgramproc> = None;
#[no_mangle]

pub static mut qglVertexAttribPointer: Option<crate::qgl_h::VertexAttribPointerproc> = None;
#[no_mangle]

pub static mut qglBindAttribLocation: Option<crate::qgl_h::BindAttribLocationproc> = None;
#[no_mangle]

pub static mut qglAttachShader: Option<crate::qgl_h::AttachShaderproc> = None;
#[no_mangle]

pub static mut qglDeleteProgram: Option<crate::qgl_h::DeleteProgramproc> = None;
#[no_mangle]

pub static mut qglDeleteShader: Option<crate::qgl_h::DeleteShaderproc> = None;
#[no_mangle]

pub static mut qglDetachShader: Option<crate::qgl_h::DetachShaderproc> = None;
#[no_mangle]

pub static mut qglDisableVertexAttribArray: Option<crate::qgl_h::DisableVertexAttribArrayproc> =
    None;
#[no_mangle]

pub static mut qglEnableVertexAttribArray: Option<crate::qgl_h::EnableVertexAttribArrayproc> = None;
#[no_mangle]

pub static mut qglGetActiveUniform: Option<crate::qgl_h::GetActiveUniformproc> = None;
#[no_mangle]

pub static mut qglGetProgramiv: Option<crate::qgl_h::GetProgramivproc> = None;
#[no_mangle]

pub static mut qglGetProgramInfoLog: Option<crate::qgl_h::GetProgramInfoLogproc> = None;
#[no_mangle]

pub static mut qglGetShaderiv: Option<crate::qgl_h::GetShaderivproc> = None;
#[no_mangle]

pub static mut qglGetShaderInfoLog: Option<crate::qgl_h::GetShaderInfoLogproc> = None;
#[no_mangle]

pub static mut qglValidateProgram: Option<crate::qgl_h::ValidateProgramproc> = None;
#[no_mangle]

pub static mut qglGetUniformLocation: Option<crate::qgl_h::GetUniformLocationproc> = None;
#[no_mangle]

pub static mut qglLinkProgram: Option<crate::qgl_h::LinkProgramproc> = None;
#[no_mangle]

pub static mut qglShaderSource: Option<crate::qgl_h::ShaderSourceproc> = None;
#[no_mangle]

pub static mut qglUseProgram: Option<crate::qgl_h::UseProgramproc> = None;
#[no_mangle]

pub static mut qglUniform1f: Option<crate::qgl_h::Uniform1fproc> = None;
#[no_mangle]

pub static mut qglUniform2f: Option<crate::qgl_h::Uniform2fproc> = None;
#[no_mangle]

pub static mut qglUniform3f: Option<crate::qgl_h::Uniform3fproc> = None;
#[no_mangle]

pub static mut qglUniform4f: Option<crate::qgl_h::Uniform4fproc> = None;
#[no_mangle]

pub static mut qglUniform1i: Option<crate::qgl_h::Uniform1iproc> = None;
#[no_mangle]

pub static mut qglUniform1fv: Option<crate::qgl_h::Uniform1fvproc> = None;
#[no_mangle]

pub static mut qglUniformMatrix4fv: Option<crate::qgl_h::UniformMatrix4fvproc> = None;
#[no_mangle]

pub static mut qglGetStringi: Option<crate::qgl_h::GetStringiproc> = None;
#[no_mangle]

pub static mut qglGetQueryObjectuiv: Option<crate::qgl_h::GetQueryObjectuivproc> = None;
#[no_mangle]

pub static mut qglGenQueries: Option<crate::qgl_h::GenQueriesproc> = None;
#[no_mangle]

pub static mut qglDeleteQueries: Option<crate::qgl_h::DeleteQueriesproc> = None;
#[no_mangle]

pub static mut qglBeginQuery: Option<crate::qgl_h::BeginQueryproc> = None;
#[no_mangle]

pub static mut qglEndQuery: Option<crate::qgl_h::EndQueryproc> = None;
#[no_mangle]

pub static mut qglGetQueryObjectiv: Option<crate::qgl_h::GetQueryObjectivproc> = None;
#[no_mangle]

pub static mut qglCheckFramebufferStatus: Option<crate::qgl_h::CheckFramebufferStatusproc> = None;
#[no_mangle]

pub static mut qglDeleteRenderbuffers: Option<crate::qgl_h::DeleteRenderbuffersproc> = None;
#[no_mangle]

pub static mut qglGenRenderbuffers: Option<crate::qgl_h::GenRenderbuffersproc> = None;
#[no_mangle]

pub static mut qglDeleteFramebuffers: Option<crate::qgl_h::DeleteFramebuffersproc> = None;
#[no_mangle]

pub static mut qglGenFramebuffers: Option<crate::qgl_h::GenFramebuffersproc> = None;
#[no_mangle]

pub static mut qglRenderbufferStorage: Option<crate::qgl_h::RenderbufferStorageproc> = None;
#[no_mangle]

pub static mut qglFramebufferTexture2D: Option<crate::qgl_h::FramebufferTexture2Dproc> = None;
#[no_mangle]

pub static mut qglFramebufferRenderbuffer: Option<crate::qgl_h::FramebufferRenderbufferproc> = None;
#[no_mangle]

pub static mut qglGenerateMipmap: Option<crate::qgl_h::GenerateMipmapproc> = None;
#[no_mangle]

pub static mut qglBlitFramebuffer: Option<crate::qgl_h::BlitFramebufferproc> = None;
#[no_mangle]

pub static mut qglRenderbufferStorageMultisample: Option<
    crate::qgl_h::RenderbufferStorageMultisampleproc,
> = None;
#[no_mangle]

pub static mut qglBindFramebuffer: Option<crate::qgl_h::BindFramebufferproc> = None;
#[no_mangle]

pub static mut qglBindRenderbuffer: Option<crate::qgl_h::BindRenderbufferproc> = None;
#[no_mangle]

pub static mut qglGenVertexArrays: Option<crate::qgl_h::GenVertexArraysproc> = None;
#[no_mangle]

pub static mut qglDeleteVertexArrays: Option<crate::qgl_h::DeleteVertexArraysproc> = None;
#[no_mangle]

pub static mut qglBindVertexArray: Option<crate::qgl_h::BindVertexArrayproc> = None;
#[no_mangle]

pub static mut qglNamedFramebufferTexture2DEXT: Option<
    crate::qgl_h::NamedFramebufferTexture2DEXTproc,
> = None;
#[no_mangle]

pub static mut qglTextureImage2DEXT: Option<crate::qgl_h::TextureImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglBindMultiTextureEXT: Option<crate::qgl_h::BindMultiTextureEXTproc> = None;
#[no_mangle]

pub static mut qglTextureParameterfEXT: Option<crate::qgl_h::TextureParameterfEXTproc> = None;
#[no_mangle]

pub static mut qglTextureParameteriEXT: Option<crate::qgl_h::TextureParameteriEXTproc> = None;
#[no_mangle]

pub static mut qglTextureSubImage2DEXT: Option<crate::qgl_h::TextureSubImage2DEXTproc> = None;
#[no_mangle]

pub static mut qglCopyTextureSubImage2DEXT: Option<crate::qgl_h::CopyTextureSubImage2DEXTproc> =
    None;
#[no_mangle]

pub static mut qglCompressedTextureImage2DEXT: Option<
    crate::qgl_h::CompressedTextureImage2DEXTproc,
> = None;
#[no_mangle]

pub static mut qglNamedFramebufferRenderbufferEXT: Option<
    crate::qgl_h::NamedFramebufferRenderbufferEXTproc,
> = None;
#[no_mangle]

pub static mut qglCompressedTextureSubImage2DEXT: Option<
    crate::qgl_h::CompressedTextureSubImage2DEXTproc,
> = None;
#[no_mangle]

pub static mut qglCheckNamedFramebufferStatusEXT: Option<
    crate::qgl_h::CheckNamedFramebufferStatusEXTproc,
> = None;
#[no_mangle]

pub static mut qglNamedRenderbufferStorageMultisampleEXT: Option<
    crate::qgl_h::NamedRenderbufferStorageMultisampleEXTproc,
> = None;
#[no_mangle]

pub static mut qglNamedRenderbufferStorageEXT: Option<
    crate::qgl_h::NamedRenderbufferStorageEXTproc,
> = None;
#[no_mangle]

pub static mut qglProgramUniformMatrix4fvEXT: Option<crate::qgl_h::ProgramUniformMatrix4fvEXTproc> =
    None;
#[no_mangle]

pub static mut qglProgramUniform1fvEXT: Option<crate::qgl_h::ProgramUniform1fvEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform4fEXT: Option<crate::qgl_h::ProgramUniform4fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform3fEXT: Option<crate::qgl_h::ProgramUniform3fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform2fEXT: Option<crate::qgl_h::ProgramUniform2fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform1fEXT: Option<crate::qgl_h::ProgramUniform1fEXTproc> = None;
#[no_mangle]

pub static mut qglProgramUniform1iEXT: Option<crate::qgl_h::ProgramUniform1iEXTproc> = None;
#[no_mangle]

pub static mut qglGenerateTextureMipmapEXT: Option<crate::qgl_h::GenerateTextureMipmapEXTproc> =
    None;
/*
===============
GLimp_Shutdown
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Shutdown() {
    crate::src::renderergl1::tr_main::ri
        .IN_Shutdown
        .expect("non-null function pointer")();
    crate::stdlib::SDL_QuitSubSystem(0x20);
}
/*
===============
GLimp_Minimize

Minimize the game so that user is back at the desktop
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Minimize() {
    crate::stdlib::SDL_MinimizeWindow(SDL_window);
}
/*
===============
GLimp_LogComment
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_LogComment(mut comment: *mut i8) {}
/*
===============
GLimp_CompareModes
===============
*/

unsafe extern "C" fn GLimp_CompareModes(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> i32 {
    let ASPECT_EPSILON: f32 = 0.001;
    let mut modeA: *mut crate::stdlib::SDL_Rect = a as *mut crate::stdlib::SDL_Rect;
    let mut modeB: *mut crate::stdlib::SDL_Rect = b as *mut crate::stdlib::SDL_Rect;
    let mut aspectA: f32 = (*modeA).w as f32 / (*modeA).h as f32;
    let mut aspectB: f32 = (*modeB).w as f32 / (*modeB).h as f32;
    let mut areaA: i32 = (*modeA).w * (*modeA).h;
    let mut areaB: i32 = (*modeB).w * (*modeB).h;
    let mut aspectDiffA: f32 = crate::stdlib::fabs(
        (aspectA - crate::src::renderergl1::tr_init::displayAspect) as f64,
    ) as f32;
    let mut aspectDiffB: f32 = crate::stdlib::fabs(
        (aspectB - crate::src::renderergl1::tr_init::displayAspect) as f64,
    ) as f32;
    let mut aspectDiffsDiff: f32 = aspectDiffA - aspectDiffB;
    if aspectDiffsDiff > ASPECT_EPSILON {
        return 1i32;
    } else if aspectDiffsDiff < -ASPECT_EPSILON {
        return -(1i32);
    } else {
        return areaA - areaB;
    };
}
/*
===============
GLimp_DetectAvailableModes
===============
*/

unsafe extern "C" fn GLimp_DetectAvailableModes() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut buf: [i8; 1024] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut numSDLModes: i32 = 0;
    let mut modes: *mut crate::stdlib::SDL_Rect = 0 as *mut crate::stdlib::SDL_Rect;
    let mut numModes: i32 = 0;
    let mut windowMode: crate::stdlib::SDL_DisplayMode = crate::stdlib::SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: 0 as *mut libc::c_void,
    };
    let mut display: i32 = crate::stdlib::SDL_GetWindowDisplayIndex(SDL_window);
    if display < 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Couldn\'t get window display index, no resolutions detected: %s\n\x00" as *const u8
                as *const i8,
            crate::stdlib::SDL_GetError(),
        );
        return;
    }
    numSDLModes = crate::stdlib::SDL_GetNumDisplayModes(display);
    if crate::stdlib::SDL_GetWindowDisplayMode(SDL_window, &mut windowMode) < 0
        || numSDLModes <= 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"Couldn\'t get window display mode, no resolutions detected: %s\n\x00" as *const u8
                as *const i8,
            crate::stdlib::SDL_GetError(),
        );
        return;
    }
    modes = crate::stdlib::SDL_calloc(
        numSDLModes as crate::stddef_h::size_t,
        
        ::std::mem::size_of::<crate::stdlib::SDL_Rect>(),
    ) as *mut crate::stdlib::SDL_Rect;
    if modes.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Out of memory\x00" as *const u8 as *const i8,
        );
    }
    i = 0;
    while i < numSDLModes {
        let mut mode: crate::stdlib::SDL_DisplayMode = crate::stdlib::SDL_DisplayMode {
            format: 0,
            w: 0,
            h: 0,
            refresh_rate: 0,
            driverdata: 0 as *mut libc::c_void,
        };
        if !(crate::stdlib::SDL_GetDisplayMode(display, i, &mut mode) < 0) {
            if mode.w == 0 || mode.h == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"Display supports any resolution\n\x00" as *const u8 as *const i8,
                );
                crate::stdlib::SDL_free(modes as *mut libc::c_void);
                return;
            }
            if !(windowMode.format != mode.format) {
                // SDL can give the same resolution with different refresh rates.
                // Only list resolution once.
                j = 0;
                while j < numModes {
                    if mode.w == (*modes.offset(j as isize)).w
                        && mode.h == (*modes.offset(j as isize)).h
                    {
                        break;
                    }
                    j += 1
                }
                if !(j != numModes) {
                    (*modes.offset(numModes as isize)).w = mode.w;
                    (*modes.offset(numModes as isize)).h = mode.h;
                    numModes += 1
                }
            }
        }
        i += 1
    }
    if numModes > 1 {
        crate::stdlib::qsort(
            modes as *mut libc::c_void,
            numModes as crate::stddef_h::size_t,
            
            ::std::mem::size_of::<crate::stdlib::SDL_Rect>(),
            Some(
                GLimp_CompareModes
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> i32,
            ),
        );
    }
    i = 0;
    while i < numModes {
        let mut newModeString: *const i8 = crate::src::qcommon::q_shared::va(
            
            b"%ux%u \x00" as *const  u8 as *mut i8,
            (*modes.offset(i as isize)).w,
            (*modes.offset(i as isize)).h,
        );
        if crate::stdlib::strlen(newModeString)
            < (::std::mem::size_of::<[i8; 1024]>() as i32
                as usize)
                .wrapping_sub(crate::stdlib::strlen(buf.as_mut_ptr()))
        {
            crate::src::qcommon::q_shared::Q_strcat(
                buf.as_mut_ptr(),
                
                ::std::mem::size_of::<[i8; 1024]>() as i32,
                newModeString,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"Skipping mode %ux%u, buffer too small\n\x00" as *const u8 as *const i8,
                (*modes.offset(i as isize)).w,
                (*modes.offset(i as isize)).h,
            );
        }
        i += 1
    }
    if *buf.as_mut_ptr() != 0 {
        buf[crate::stdlib::strlen(buf.as_mut_ptr()).wrapping_sub(1usize)] = 0;
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Available modes: \'%s\'\n\x00" as *const u8 as *const i8,
            buf.as_mut_ptr(),
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_availableModes\x00" as *const u8 as *const i8,
            buf.as_mut_ptr(),
        );
    }
    crate::stdlib::SDL_free(modes as *mut libc::c_void);
}
/*
===============
GLimp_GetProcAddresses

Get addresses for OpenGL functions.
===============
*/

unsafe extern "C" fn GLimp_GetProcAddresses(
    mut fixedFunction: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut success: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qtrue;
    let mut version: *const i8 = 0 as *const i8;
    // OpenGL 1.0 and OpenGL ES 1.0
    qglGetString = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::GetStringproc>>(
        crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetString\x00" as *const u8 as *const i8,
        ),
    ); // ES, ES-CM, or ES-CL
    if qglGetString.is_none() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
            b"glGetString\x00" as *const u8 as *const i8,
        );
        success = crate::src::qcommon::q_shared::qfalse
    }
    if qglGetString.is_none() {
        crate::src::renderergl1::tr_subs::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"glGetString is NULL\x00" as *const u8 as *const i8,
        );
    }
    version = qglGetString.expect("non-null function pointer")(
        0x1f02u32,
    ) as *const i8;
    if version.is_null() {
        crate::src::renderergl1::tr_subs::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"GL_VERSION is NULL\n\x00" as *const u8 as *const i8,
        );
    }
    if crate::src::qcommon::q_shared::Q_stricmpn(
        b"OpenGL ES\x00" as *const u8 as *const i8,
        version,
        9,
    ) == 0
    {
        let mut profile: [i8; 6] = [0; 6];
        crate::stdlib::sscanf(
            version,
            b"OpenGL %5s %d.%d\x00" as *const u8 as *const i8,
            profile.as_mut_ptr(),
            &mut qglesMajorVersion as *mut i32,
            &mut qglesMinorVersion as *mut i32,
        );
        // common lite profile (no floating point) is not supported
        if crate::src::qcommon::q_shared::Q_stricmp(
            profile.as_mut_ptr(),
            b"ES-CL\x00" as *const u8 as *const i8,
        ) == 0
        {
            qglesMajorVersion = 0;
            qglesMinorVersion = 0
        }
    } else {
        crate::stdlib::sscanf(
            version,
            b"%d.%d\x00" as *const u8 as *const i8,
            &mut qglMajorVersion as *mut i32,
            &mut qglMinorVersion as *mut i32,
        );
    }
    if fixedFunction as u64 != 0 {
        if qglMajorVersion > 1
            || qglMajorVersion == 1 && qglMinorVersion >= 2
        {
            qglBindTexture = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::BindTextureproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glBindTexture\x00" as *const u8 as *const i8,
            ));
            if qglBindTexture.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glBindTexture\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglBlendFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::BlendFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glBlendFunc\x00" as *const u8 as *const i8,
            ));
            if qglBlendFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glBlendFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearColor = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearColorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearColor\x00" as *const u8 as *const i8,
            ));
            if qglClearColor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearColor\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Clearproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glClear\x00" as *const u8 as *const i8,
                ),
            );
            if qglClear.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClear\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearStencil = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearStencilproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearStencil\x00" as *const u8 as *const i8,
            ));
            if qglClearStencil.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearStencil\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColorMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ColorMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColorMask\x00" as *const u8 as *const i8,
            ));
            if qglColorMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColorMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglCopyTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::CopyTexSubImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
            ));
            if qglCopyTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglCullFace = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::CullFaceproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glCullFace\x00" as *const u8 as *const i8,
            ));
            if qglCullFace.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glCullFace\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDeleteTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DeleteTexturesproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDeleteTextures\x00" as *const u8 as *const i8,
            ));
            if qglDeleteTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDeleteTextures\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthFunc\x00" as *const u8 as *const i8,
            ));
            if qglDepthFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthMask\x00" as *const u8 as *const i8,
            ));
            if qglDepthMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDisable = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Disableproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisable\x00" as *const u8 as *const i8,
            ));
            if qglDisable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDisable\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDrawArrays = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DrawArraysproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDrawArrays\x00" as *const u8 as *const i8,
            ));
            if qglDrawArrays.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDrawArrays\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDrawElements = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DrawElementsproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDrawElements\x00" as *const u8 as *const i8,
            ));
            if qglDrawElements.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDrawElements\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Enableproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glEnable\x00" as *const u8 as *const i8,
                ),
            );
            if qglEnable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glEnable\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Finishproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glFinish\x00" as *const u8 as *const i8,
                ),
            );
            if qglFinish.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFinish\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Flushproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glFlush\x00" as *const u8 as *const i8,
                ),
            );
            if qglFlush.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFlush\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGenTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GenTexturesproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGenTextures\x00" as *const u8 as *const i8,
            ));
            if qglGenTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGenTextures\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetBooleanv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetBooleanvproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetBooleanv\x00" as *const u8 as *const i8,
            ));
            if qglGetBooleanv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetBooleanv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetError = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetErrorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetError\x00" as *const u8 as *const i8,
            ));
            if qglGetError.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetError\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetIntegerv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetIntegervproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetIntegerv\x00" as *const u8 as *const i8,
            ));
            if qglGetIntegerv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetIntegerv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetString = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetStringproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetString\x00" as *const u8 as *const i8,
            ));
            if qglGetString.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetString\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLineWidth = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LineWidthproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLineWidth\x00" as *const u8 as *const i8,
            ));
            if qglLineWidth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLineWidth\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPolygonOffset = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PolygonOffsetproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPolygonOffset\x00" as *const u8 as *const i8,
            ));
            if qglPolygonOffset.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPolygonOffset\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglReadPixels = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ReadPixelsproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glReadPixels\x00" as *const u8 as *const i8,
            ));
            if qglReadPixels.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glReadPixels\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglScissor = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Scissorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glScissor\x00" as *const u8 as *const i8,
            ));
            if qglScissor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glScissor\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilFunc\x00" as *const u8 as *const i8,
            ));
            if qglStencilFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilMask\x00" as *const u8 as *const i8,
            ));
            if qglStencilMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilOp = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilOpproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilOp\x00" as *const u8 as *const i8,
            ));
            if qglStencilOp.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilOp\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexImage2D\x00" as *const u8 as *const i8,
            ));
            if qglTexImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexParameterf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexParameterfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexParameterf\x00" as *const u8 as *const i8,
            ));
            if qglTexParameterf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexParameterf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexParameteri = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexParameteriproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexParameteri\x00" as *const u8 as *const i8,
            ));
            if qglTexParameteri.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexParameteri\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexSubImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexSubImage2D\x00" as *const u8 as *const i8,
            ));
            if qglTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexSubImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTranslatef = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Translatefproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTranslatef\x00" as *const u8 as *const i8,
            ));
            if qglTranslatef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTranslatef\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglViewport = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Viewportproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glViewport\x00" as *const u8 as *const i8,
            ));
            if qglViewport.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glViewport\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglAlphaFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::AlphaFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glAlphaFunc\x00" as *const u8 as *const i8,
            ));
            if qglAlphaFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glAlphaFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColor4f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Color4fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColor4f\x00" as *const u8 as *const i8,
            ));
            if qglColor4f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColor4f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColorPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ColorPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColorPointer\x00" as *const u8 as *const i8,
            ));
            if qglColorPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColorPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDisableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DisableClientStateproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisableClientState\x00" as *const u8 as *const i8,
            ));
            if qglDisableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDisableClientState\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglEnableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::EnableClientStateproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glEnableClientState\x00" as *const u8 as *const i8,
            ));
            if qglEnableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glEnableClientState\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLoadIdentity = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LoadIdentityproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLoadIdentity\x00" as *const u8 as *const i8,
            ));
            if qglLoadIdentity.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLoadIdentity\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLoadMatrixf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LoadMatrixfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLoadMatrixf\x00" as *const u8 as *const i8,
            ));
            if qglLoadMatrixf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLoadMatrixf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglMatrixMode = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::MatrixModeproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glMatrixMode\x00" as *const u8 as *const i8,
            ));
            if qglMatrixMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glMatrixMode\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPopMatrix = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PopMatrixproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPopMatrix\x00" as *const u8 as *const i8,
            ));
            if qglPopMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPopMatrix\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPushMatrix = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PushMatrixproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPushMatrix\x00" as *const u8 as *const i8,
            ));
            if qglPushMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPushMatrix\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglShadeModel = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ShadeModelproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glShadeModel\x00" as *const u8 as *const i8,
            ));
            if qglShadeModel.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glShadeModel\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexCoordPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexCoordPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexCoordPointer\x00" as *const u8 as *const i8,
            ));
            if qglTexCoordPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexCoordPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexEnvf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexEnvfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexEnvf\x00" as *const u8 as *const i8,
            ));
            if qglTexEnvf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexEnvf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglVertexPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::VertexPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glVertexPointer\x00" as *const u8 as *const i8,
            ));
            if qglVertexPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glVertexPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearDepth = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearDepthproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearDepth\x00" as *const u8 as *const i8,
            ));
            if qglClearDepth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearDepth\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthRange = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthRangeproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthRange\x00" as *const u8 as *const i8,
            ));
            if qglDepthRange.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthRange\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDrawBuffer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DrawBufferproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDrawBuffer\x00" as *const u8 as *const i8,
            ));
            if qglDrawBuffer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDrawBuffer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPolygonMode = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PolygonModeproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPolygonMode\x00" as *const u8 as *const i8,
            ));
            if qglPolygonMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPolygonMode\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglArrayElement = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ArrayElementproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glArrayElement\x00" as *const u8 as *const i8,
            ));
            if qglArrayElement.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glArrayElement\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglBegin = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Beginproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glBegin\x00" as *const u8 as *const i8,
                ),
            );
            if qglBegin.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glBegin\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClipPlane = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClipPlaneproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClipPlane\x00" as *const u8 as *const i8,
            ));
            if qglClipPlane.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClipPlane\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColor3f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Color3fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColor3f\x00" as *const u8 as *const i8,
            ));
            if qglColor3f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColor3f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColor4ubv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Color4ubvproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColor4ubv\x00" as *const u8 as *const i8,
            ));
            if qglColor4ubv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColor4ubv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglEnd = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Endproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glEnd\x00" as *const u8 as *const i8,
                ),
            );
            if qglEnd.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glEnd\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFrustum = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Frustumproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glFrustum\x00" as *const u8 as *const i8,
            ));
            if qglFrustum.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFrustum\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglOrtho = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Orthoproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glOrtho\x00" as *const u8 as *const i8,
                ),
            );
            if qglOrtho.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glOrtho\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexCoord2f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexCoord2fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexCoord2f\x00" as *const u8 as *const i8,
            ));
            if qglTexCoord2f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexCoord2f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexCoord2fv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexCoord2fvproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexCoord2fv\x00" as *const u8 as *const i8,
            ));
            if qglTexCoord2fv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexCoord2fv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglVertex2f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Vertex2fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glVertex2f\x00" as *const u8 as *const i8,
            ));
            if qglVertex2f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glVertex2f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglVertex3f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Vertex3fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glVertex3f\x00" as *const u8 as *const i8,
            ));
            if qglVertex3f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glVertex3f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglVertex3fv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Vertex3fvproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glVertex3fv\x00" as *const u8 as *const i8,
            ));
            if qglVertex3fv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glVertex3fv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
        } else if qglesMajorVersion == 1 && qglesMinorVersion >= 1 {
            // OpenGL ES 1.1 (2.0 is not backward compatible)
            qglBindTexture = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::BindTextureproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glBindTexture\x00" as *const u8 as *const i8,
            ));
            if qglBindTexture.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glBindTexture\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglBlendFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::BlendFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glBlendFunc\x00" as *const u8 as *const i8,
            ));
            if qglBlendFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glBlendFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearColor = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearColorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearColor\x00" as *const u8 as *const i8,
            ));
            if qglClearColor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearColor\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Clearproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glClear\x00" as *const u8 as *const i8,
                ),
            );
            if qglClear.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClear\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearStencil = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearStencilproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearStencil\x00" as *const u8 as *const i8,
            ));
            if qglClearStencil.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearStencil\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColorMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ColorMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColorMask\x00" as *const u8 as *const i8,
            ));
            if qglColorMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColorMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglCopyTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::CopyTexSubImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
            ));
            if qglCopyTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglCullFace = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::CullFaceproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glCullFace\x00" as *const u8 as *const i8,
            ));
            if qglCullFace.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glCullFace\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDeleteTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DeleteTexturesproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDeleteTextures\x00" as *const u8 as *const i8,
            ));
            if qglDeleteTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDeleteTextures\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthFunc\x00" as *const u8 as *const i8,
            ));
            if qglDepthFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthMask\x00" as *const u8 as *const i8,
            ));
            if qglDepthMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDisable = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Disableproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisable\x00" as *const u8 as *const i8,
            ));
            if qglDisable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDisable\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDrawArrays = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DrawArraysproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDrawArrays\x00" as *const u8 as *const i8,
            ));
            if qglDrawArrays.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDrawArrays\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDrawElements = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DrawElementsproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDrawElements\x00" as *const u8 as *const i8,
            ));
            if qglDrawElements.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDrawElements\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Enableproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glEnable\x00" as *const u8 as *const i8,
                ),
            );
            if qglEnable.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glEnable\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Finishproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glFinish\x00" as *const u8 as *const i8,
                ),
            );
            if qglFinish.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFinish\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Flushproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glFlush\x00" as *const u8 as *const i8,
                ),
            );
            if qglFlush.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFlush\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGenTextures = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GenTexturesproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGenTextures\x00" as *const u8 as *const i8,
            ));
            if qglGenTextures.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGenTextures\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetBooleanv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetBooleanvproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetBooleanv\x00" as *const u8 as *const i8,
            ));
            if qglGetBooleanv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetBooleanv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetError = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetErrorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetError\x00" as *const u8 as *const i8,
            ));
            if qglGetError.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetError\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetIntegerv = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetIntegervproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetIntegerv\x00" as *const u8 as *const i8,
            ));
            if qglGetIntegerv.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetIntegerv\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglGetString = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::GetStringproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetString\x00" as *const u8 as *const i8,
            ));
            if qglGetString.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glGetString\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLineWidth = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LineWidthproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLineWidth\x00" as *const u8 as *const i8,
            ));
            if qglLineWidth.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLineWidth\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPolygonOffset = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PolygonOffsetproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPolygonOffset\x00" as *const u8 as *const i8,
            ));
            if qglPolygonOffset.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPolygonOffset\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglReadPixels = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ReadPixelsproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glReadPixels\x00" as *const u8 as *const i8,
            ));
            if qglReadPixels.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glReadPixels\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglScissor = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Scissorproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glScissor\x00" as *const u8 as *const i8,
            ));
            if qglScissor.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glScissor\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilFunc\x00" as *const u8 as *const i8,
            ));
            if qglStencilFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilMask = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilMaskproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilMask\x00" as *const u8 as *const i8,
            ));
            if qglStencilMask.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilMask\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglStencilOp = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::StencilOpproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glStencilOp\x00" as *const u8 as *const i8,
            ));
            if qglStencilOp.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glStencilOp\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexImage2D\x00" as *const u8 as *const i8,
            ));
            if qglTexImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexParameterf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexParameterfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexParameterf\x00" as *const u8 as *const i8,
            ));
            if qglTexParameterf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexParameterf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexParameteri = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexParameteriproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexParameteri\x00" as *const u8 as *const i8,
            ));
            if qglTexParameteri.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexParameteri\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexSubImage2D = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexSubImage2Dproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexSubImage2D\x00" as *const u8 as *const i8,
            ));
            if qglTexSubImage2D.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexSubImage2D\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTranslatef = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Translatefproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTranslatef\x00" as *const u8 as *const i8,
            ));
            if qglTranslatef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTranslatef\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglViewport = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Viewportproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glViewport\x00" as *const u8 as *const i8,
            ));
            if qglViewport.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glViewport\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglAlphaFunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::AlphaFuncproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glAlphaFunc\x00" as *const u8 as *const i8,
            ));
            if qglAlphaFunc.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glAlphaFunc\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColor4f = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Color4fproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColor4f\x00" as *const u8 as *const i8,
            ));
            if qglColor4f.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColor4f\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglColorPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ColorPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glColorPointer\x00" as *const u8 as *const i8,
            ));
            if qglColorPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glColorPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDisableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DisableClientStateproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisableClientState\x00" as *const u8 as *const i8,
            ));
            if qglDisableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDisableClientState\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglEnableClientState = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::EnableClientStateproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glEnableClientState\x00" as *const u8 as *const i8,
            ));
            if qglEnableClientState.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glEnableClientState\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLoadIdentity = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LoadIdentityproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLoadIdentity\x00" as *const u8 as *const i8,
            ));
            if qglLoadIdentity.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLoadIdentity\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglLoadMatrixf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::LoadMatrixfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glLoadMatrixf\x00" as *const u8 as *const i8,
            ));
            if qglLoadMatrixf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glLoadMatrixf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglMatrixMode = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::MatrixModeproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glMatrixMode\x00" as *const u8 as *const i8,
            ));
            if qglMatrixMode.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glMatrixMode\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPopMatrix = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PopMatrixproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPopMatrix\x00" as *const u8 as *const i8,
            ));
            if qglPopMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPopMatrix\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglPushMatrix = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::PushMatrixproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glPushMatrix\x00" as *const u8 as *const i8,
            ));
            if qglPushMatrix.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glPushMatrix\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglShadeModel = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ShadeModelproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glShadeModel\x00" as *const u8 as *const i8,
            ));
            if qglShadeModel.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glShadeModel\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexCoordPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexCoordPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexCoordPointer\x00" as *const u8 as *const i8,
            ));
            if qglTexCoordPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexCoordPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglTexEnvf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::TexEnvfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glTexEnvf\x00" as *const u8 as *const i8,
            ));
            if qglTexEnvf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glTexEnvf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglVertexPointer = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::VertexPointerproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glVertexPointer\x00" as *const u8 as *const i8,
            ));
            if qglVertexPointer.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glVertexPointer\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClearDepthf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClearDepthfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClearDepthf\x00" as *const u8 as *const i8,
            ));
            if qglClearDepthf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClearDepthf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglDepthRangef = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::DepthRangefproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glDepthRangef\x00" as *const u8 as *const i8,
            ));
            if qglDepthRangef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glDepthRangef\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglClipPlanef = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::ClipPlanefproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glClipPlanef\x00" as *const u8 as *const i8,
            ));
            if qglClipPlanef.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glClipPlanef\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglFrustumf = ::std::mem::transmute::<
                *mut libc::c_void,
                Option<crate::qgl_h::Frustumfproc>,
            >(crate::stdlib::SDL_GL_GetProcAddress(
                b"glFrustumf\x00" as *const u8 as *const i8,
            ));
            if qglFrustumf.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glFrustumf\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            qglOrthof = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Orthofproc>>(
                crate::stdlib::SDL_GL_GetProcAddress(
                    b"glOrthof\x00" as *const u8 as *const i8,
                ),
            );
            if qglOrthof.is_none() {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                    b"glOrthof\x00" as *const u8 as *const i8,
                );
                success = crate::src::qcommon::q_shared::qfalse
            }
            // error so this doesn't segfault due to NULL desktop GL functions being used
            crate::src::renderergl1::tr_subs::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"Unsupported OpenGL Version: %s\n\x00" as *const u8 as *const i8,
                version,
            );
        } else {
            crate::src::renderergl1::tr_subs::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"Unsupported OpenGL Version (%s), OpenGL 1.2 is required\n\x00" as *const u8
                    as *const i8,
                version,
            );
        }
    } else if qglMajorVersion > 2
        || qglMajorVersion == 2 && qglMinorVersion >= 0
    {
        qglBindTexture = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindTextureproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindTexture\x00" as *const u8 as *const i8,
        ));
        if qglBindTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindTexture\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBlendFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BlendFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBlendFunc\x00" as *const u8 as *const i8,
        ));
        if qglBlendFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBlendFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearColor = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearColorproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearColor\x00" as *const u8 as *const i8,
        ));
        if qglClearColor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearColor\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Clearproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glClear\x00" as *const u8 as *const i8,
            ),
        );
        if qglClear.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClear\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearStencil = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearStencilproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearStencil\x00" as *const u8 as *const i8,
        ));
        if qglClearStencil.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearStencil\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglColorMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ColorMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glColorMask\x00" as *const u8 as *const i8,
        ));
        if qglColorMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glColorMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCopyTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CopyTexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCopyTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::CullFaceproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glCullFace\x00" as *const u8 as *const i8,
            ),
        );
        if qglCullFace.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCullFace\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteTextures = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteTexturesproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteTextures\x00" as *const u8 as *const i8,
        ));
        if qglDeleteTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteTextures\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthFunc\x00" as *const u8 as *const i8,
        ));
        if qglDepthFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthMask\x00" as *const u8 as *const i8,
        ));
        if qglDepthMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Disableproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisable\x00" as *const u8 as *const i8,
            ),
        );
        if qglDisable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDisable\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDrawArrays = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DrawArraysproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDrawArrays\x00" as *const u8 as *const i8,
        ));
        if qglDrawArrays.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDrawArrays\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDrawElements = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DrawElementsproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDrawElements\x00" as *const u8 as *const i8,
        ));
        if qglDrawElements.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDrawElements\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Enableproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glEnable\x00" as *const u8 as *const i8,
            ),
        );
        if qglEnable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glEnable\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Finishproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glFinish\x00" as *const u8 as *const i8,
            ),
        );
        if qglFinish.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glFinish\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Flushproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glFlush\x00" as *const u8 as *const i8,
            ),
        );
        if qglFlush.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glFlush\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGenTextures = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GenTexturesproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGenTextures\x00" as *const u8 as *const i8,
        ));
        if qglGenTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGenTextures\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetBooleanv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetBooleanvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetBooleanv\x00" as *const u8 as *const i8,
        ));
        if qglGetBooleanv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetBooleanv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::GetErrorproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetError\x00" as *const u8 as *const i8,
            ),
        );
        if qglGetError.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetError\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetIntegerv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetIntegervproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetIntegerv\x00" as *const u8 as *const i8,
        ));
        if qglGetIntegerv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetIntegerv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetString = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetStringproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetString\x00" as *const u8 as *const i8,
        ));
        if qglGetString.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetString\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglLineWidth = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::LineWidthproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glLineWidth\x00" as *const u8 as *const i8,
        ));
        if qglLineWidth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glLineWidth\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglPolygonOffset = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::PolygonOffsetproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glPolygonOffset\x00" as *const u8 as *const i8,
        ));
        if qglPolygonOffset.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glPolygonOffset\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglReadPixels = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ReadPixelsproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glReadPixels\x00" as *const u8 as *const i8,
        ));
        if qglReadPixels.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glReadPixels\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Scissorproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glScissor\x00" as *const u8 as *const i8,
            ),
        );
        if qglScissor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glScissor\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilFunc\x00" as *const u8 as *const i8,
        ));
        if qglStencilFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilMask\x00" as *const u8 as *const i8,
        ));
        if qglStencilMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilOp = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilOpproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilOp\x00" as *const u8 as *const i8,
        ));
        if qglStencilOp.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilOp\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexImage2D\x00" as *const u8 as *const i8,
        ));
        if qglTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexParameterf = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexParameterfproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexParameterf\x00" as *const u8 as *const i8,
        ));
        if qglTexParameterf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexParameterf\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexParameteri = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexParameteriproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexParameteri\x00" as *const u8 as *const i8,
        ));
        if qglTexParameteri.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexParameteri\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTranslatef = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Translatefproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTranslatef\x00" as *const u8 as *const i8,
        ));
        if qglTranslatef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTranslatef\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Viewportproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glViewport\x00" as *const u8 as *const i8,
            ),
        );
        if qglViewport.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glViewport\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearDepth = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearDepthproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearDepth\x00" as *const u8 as *const i8,
        ));
        if qglClearDepth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearDepth\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthRange = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthRangeproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthRange\x00" as *const u8 as *const i8,
        ));
        if qglDepthRange.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthRange\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDrawBuffer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DrawBufferproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDrawBuffer\x00" as *const u8 as *const i8,
        ));
        if qglDrawBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDrawBuffer\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglPolygonMode = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::PolygonModeproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glPolygonMode\x00" as *const u8 as *const i8,
        ));
        if qglPolygonMode.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glPolygonMode\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglActiveTexture = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ActiveTextureproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glActiveTexture\x00" as *const u8 as *const i8,
        ));
        if qglActiveTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glActiveTexture\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompressedTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompressedTexImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompressedTexImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCompressedTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompressedTexImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompressedTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompressedTexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompressedTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCompressedTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompressedTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBindBuffer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindBufferproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindBuffer\x00" as *const u8 as *const i8,
        ));
        if qglBindBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindBuffer\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteBuffers = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteBuffersproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteBuffers\x00" as *const u8 as *const i8,
        ));
        if qglDeleteBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteBuffers\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGenBuffers = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GenBuffersproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGenBuffers\x00" as *const u8 as *const i8,
        ));
        if qglGenBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGenBuffers\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBufferData = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BufferDataproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBufferData\x00" as *const u8 as *const i8,
        ));
        if qglBufferData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBufferData\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBufferSubData = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BufferSubDataproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBufferSubData\x00" as *const u8 as *const i8,
        ));
        if qglBufferSubData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBufferSubData\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglAttachShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::AttachShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glAttachShader\x00" as *const u8 as *const i8,
        ));
        if qglAttachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glAttachShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBindAttribLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindAttribLocationproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindAttribLocation\x00" as *const u8 as *const i8,
        ));
        if qglBindAttribLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindAttribLocation\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompileShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompileShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompileShader\x00" as *const u8 as *const i8,
        ));
        if qglCompileShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompileShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCreateProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CreateProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCreateProgram\x00" as *const u8 as *const i8,
        ));
        if qglCreateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCreateProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCreateShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CreateShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCreateShader\x00" as *const u8 as *const i8,
        ));
        if qglCreateShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCreateShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteProgram\x00" as *const u8 as *const i8,
        ));
        if qglDeleteProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteShader\x00" as *const u8 as *const i8,
        ));
        if qglDeleteShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDetachShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DetachShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDetachShader\x00" as *const u8 as *const i8,
        ));
        if qglDetachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDetachShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDisableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DisableVertexAttribArrayproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDisableVertexAttribArray\x00" as *const u8 as *const i8,
        ));
        if qglDisableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDisableVertexAttribArray\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglEnableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::EnableVertexAttribArrayproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glEnableVertexAttribArray\x00" as *const u8 as *const i8,
        ));
        if qglEnableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glEnableVertexAttribArray\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetActiveUniform = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetActiveUniformproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetActiveUniform\x00" as *const u8 as *const i8,
        ));
        if qglGetActiveUniform.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetActiveUniform\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetProgramiv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetProgramivproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetProgramiv\x00" as *const u8 as *const i8,
        ));
        if qglGetProgramiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetProgramiv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetProgramInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetProgramInfoLogproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetProgramInfoLog\x00" as *const u8 as *const i8,
        ));
        if qglGetProgramInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetProgramInfoLog\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderiv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderivproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderiv\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderiv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderInfoLogproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderInfoLog\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderInfoLog\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderSource = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderSourceproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderSource\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderSource\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetUniformLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetUniformLocationproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetUniformLocation\x00" as *const u8 as *const i8,
        ));
        if qglGetUniformLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetUniformLocation\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglLinkProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::LinkProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glLinkProgram\x00" as *const u8 as *const i8,
        ));
        if qglLinkProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glLinkProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglShaderSource = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ShaderSourceproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glShaderSource\x00" as *const u8 as *const i8,
        ));
        if qglShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glShaderSource\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUseProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::UseProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUseProgram\x00" as *const u8 as *const i8,
        ));
        if qglUseProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUseProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1f\x00" as *const u8 as *const i8,
        ));
        if qglUniform1f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform2f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform2fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform2f\x00" as *const u8 as *const i8,
        ));
        if qglUniform2f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform2f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform3f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform3fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform3f\x00" as *const u8 as *const i8,
        ));
        if qglUniform3f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform3f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform4f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform4fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform4f\x00" as *const u8 as *const i8,
        ));
        if qglUniform4f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform4f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1i = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1iproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1i\x00" as *const u8 as *const i8,
        ));
        if qglUniform1i.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1i\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1fvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1fv\x00" as *const u8 as *const i8,
        ));
        if qglUniform1fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1fv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniformMatrix4fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::UniformMatrix4fvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniformMatrix4fv\x00" as *const u8 as *const i8,
        ));
        if qglUniformMatrix4fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniformMatrix4fv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglValidateProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ValidateProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glValidateProgram\x00" as *const u8 as *const i8,
        ));
        if qglValidateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glValidateProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglVertexAttribPointer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::VertexAttribPointerproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glVertexAttribPointer\x00" as *const u8 as *const i8,
        ));
        if qglVertexAttribPointer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glVertexAttribPointer\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
    } else if qglesMajorVersion > 2
        || qglesMajorVersion == 2 && qglesMinorVersion >= 0
    {
        qglBindTexture = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindTextureproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindTexture\x00" as *const u8 as *const i8,
        ));
        if qglBindTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindTexture\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBlendFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BlendFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBlendFunc\x00" as *const u8 as *const i8,
        ));
        if qglBlendFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBlendFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearColor = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearColorproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearColor\x00" as *const u8 as *const i8,
        ));
        if qglClearColor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearColor\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClear = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Clearproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glClear\x00" as *const u8 as *const i8,
            ),
        );
        if qglClear.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClear\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearStencil = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearStencilproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearStencil\x00" as *const u8 as *const i8,
        ));
        if qglClearStencil.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearStencil\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglColorMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ColorMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glColorMask\x00" as *const u8 as *const i8,
        ));
        if qglColorMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glColorMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCopyTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CopyTexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCopyTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCopyTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCullFace = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::CullFaceproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glCullFace\x00" as *const u8 as *const i8,
            ),
        );
        if qglCullFace.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCullFace\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteTextures = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteTexturesproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteTextures\x00" as *const u8 as *const i8,
        ));
        if qglDeleteTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteTextures\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthFunc\x00" as *const u8 as *const i8,
        ));
        if qglDepthFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthMask\x00" as *const u8 as *const i8,
        ));
        if qglDepthMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDisable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Disableproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glDisable\x00" as *const u8 as *const i8,
            ),
        );
        if qglDisable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDisable\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDrawArrays = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DrawArraysproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDrawArrays\x00" as *const u8 as *const i8,
        ));
        if qglDrawArrays.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDrawArrays\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDrawElements = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DrawElementsproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDrawElements\x00" as *const u8 as *const i8,
        ));
        if qglDrawElements.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDrawElements\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglEnable = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Enableproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glEnable\x00" as *const u8 as *const i8,
            ),
        );
        if qglEnable.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glEnable\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglFinish = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Finishproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glFinish\x00" as *const u8 as *const i8,
            ),
        );
        if qglFinish.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glFinish\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglFlush = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Flushproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glFlush\x00" as *const u8 as *const i8,
            ),
        );
        if qglFlush.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glFlush\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGenTextures = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GenTexturesproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGenTextures\x00" as *const u8 as *const i8,
        ));
        if qglGenTextures.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGenTextures\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetBooleanv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetBooleanvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetBooleanv\x00" as *const u8 as *const i8,
        ));
        if qglGetBooleanv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetBooleanv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetError = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::GetErrorproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glGetError\x00" as *const u8 as *const i8,
            ),
        );
        if qglGetError.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetError\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetIntegerv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetIntegervproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetIntegerv\x00" as *const u8 as *const i8,
        ));
        if qglGetIntegerv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetIntegerv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetString = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetStringproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetString\x00" as *const u8 as *const i8,
        ));
        if qglGetString.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetString\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglLineWidth = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::LineWidthproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glLineWidth\x00" as *const u8 as *const i8,
        ));
        if qglLineWidth.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glLineWidth\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglPolygonOffset = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::PolygonOffsetproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glPolygonOffset\x00" as *const u8 as *const i8,
        ));
        if qglPolygonOffset.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glPolygonOffset\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglReadPixels = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ReadPixelsproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glReadPixels\x00" as *const u8 as *const i8,
        ));
        if qglReadPixels.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glReadPixels\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglScissor = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Scissorproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glScissor\x00" as *const u8 as *const i8,
            ),
        );
        if qglScissor.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glScissor\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilFunc = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilFuncproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilFunc\x00" as *const u8 as *const i8,
        ));
        if qglStencilFunc.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilFunc\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilMask = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilMaskproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilMask\x00" as *const u8 as *const i8,
        ));
        if qglStencilMask.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilMask\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglStencilOp = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::StencilOpproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glStencilOp\x00" as *const u8 as *const i8,
        ));
        if qglStencilOp.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glStencilOp\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexImage2D\x00" as *const u8 as *const i8,
        ));
        if qglTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexParameterf = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexParameterfproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexParameterf\x00" as *const u8 as *const i8,
        ));
        if qglTexParameterf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexParameterf\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexParameteri = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexParameteriproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexParameteri\x00" as *const u8 as *const i8,
        ));
        if qglTexParameteri.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexParameteri\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::TexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglTranslatef = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Translatefproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glTranslatef\x00" as *const u8 as *const i8,
        ));
        if qglTranslatef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glTranslatef\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglViewport = ::std::mem::transmute::<*mut libc::c_void, Option<crate::qgl_h::Viewportproc>>(
            crate::stdlib::SDL_GL_GetProcAddress(
                b"glViewport\x00" as *const u8 as *const i8,
            ),
        );
        if qglViewport.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glViewport\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglClearDepthf = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ClearDepthfproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glClearDepthf\x00" as *const u8 as *const i8,
        ));
        if qglClearDepthf.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glClearDepthf\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDepthRangef = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DepthRangefproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDepthRangef\x00" as *const u8 as *const i8,
        ));
        if qglDepthRangef.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDepthRangef\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglActiveTexture = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ActiveTextureproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glActiveTexture\x00" as *const u8 as *const i8,
        ));
        if qglActiveTexture.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glActiveTexture\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompressedTexImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompressedTexImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompressedTexImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCompressedTexImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompressedTexImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompressedTexSubImage2D = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompressedTexSubImage2Dproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompressedTexSubImage2D\x00" as *const u8 as *const i8,
        ));
        if qglCompressedTexSubImage2D.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompressedTexSubImage2D\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBindBuffer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindBufferproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindBuffer\x00" as *const u8 as *const i8,
        ));
        if qglBindBuffer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindBuffer\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteBuffers = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteBuffersproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteBuffers\x00" as *const u8 as *const i8,
        ));
        if qglDeleteBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteBuffers\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGenBuffers = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GenBuffersproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGenBuffers\x00" as *const u8 as *const i8,
        ));
        if qglGenBuffers.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGenBuffers\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBufferData = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BufferDataproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBufferData\x00" as *const u8 as *const i8,
        ));
        if qglBufferData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBufferData\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBufferSubData = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BufferSubDataproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBufferSubData\x00" as *const u8 as *const i8,
        ));
        if qglBufferSubData.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBufferSubData\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglAttachShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::AttachShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glAttachShader\x00" as *const u8 as *const i8,
        ));
        if qglAttachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glAttachShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglBindAttribLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::BindAttribLocationproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glBindAttribLocation\x00" as *const u8 as *const i8,
        ));
        if qglBindAttribLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glBindAttribLocation\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCompileShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CompileShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCompileShader\x00" as *const u8 as *const i8,
        ));
        if qglCompileShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCompileShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCreateProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CreateProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCreateProgram\x00" as *const u8 as *const i8,
        ));
        if qglCreateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCreateProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglCreateShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::CreateShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glCreateShader\x00" as *const u8 as *const i8,
        ));
        if qglCreateShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glCreateShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteProgram\x00" as *const u8 as *const i8,
        ));
        if qglDeleteProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDeleteShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DeleteShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDeleteShader\x00" as *const u8 as *const i8,
        ));
        if qglDeleteShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDeleteShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDetachShader = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DetachShaderproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDetachShader\x00" as *const u8 as *const i8,
        ));
        if qglDetachShader.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDetachShader\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglDisableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::DisableVertexAttribArrayproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glDisableVertexAttribArray\x00" as *const u8 as *const i8,
        ));
        if qglDisableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glDisableVertexAttribArray\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglEnableVertexAttribArray = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::EnableVertexAttribArrayproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glEnableVertexAttribArray\x00" as *const u8 as *const i8,
        ));
        if qglEnableVertexAttribArray.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glEnableVertexAttribArray\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetActiveUniform = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetActiveUniformproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetActiveUniform\x00" as *const u8 as *const i8,
        ));
        if qglGetActiveUniform.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetActiveUniform\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetProgramiv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetProgramivproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetProgramiv\x00" as *const u8 as *const i8,
        ));
        if qglGetProgramiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetProgramiv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetProgramInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetProgramInfoLogproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetProgramInfoLog\x00" as *const u8 as *const i8,
        ));
        if qglGetProgramInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetProgramInfoLog\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderiv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderivproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderiv\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderiv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderiv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderInfoLog = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderInfoLogproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderInfoLog\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderInfoLog.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderInfoLog\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetShaderSource = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetShaderSourceproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetShaderSource\x00" as *const u8 as *const i8,
        ));
        if qglGetShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetShaderSource\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglGetUniformLocation = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetUniformLocationproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetUniformLocation\x00" as *const u8 as *const i8,
        ));
        if qglGetUniformLocation.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetUniformLocation\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglLinkProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::LinkProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glLinkProgram\x00" as *const u8 as *const i8,
        ));
        if qglLinkProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glLinkProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglShaderSource = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ShaderSourceproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glShaderSource\x00" as *const u8 as *const i8,
        ));
        if qglShaderSource.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glShaderSource\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUseProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::UseProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUseProgram\x00" as *const u8 as *const i8,
        ));
        if qglUseProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUseProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1f\x00" as *const u8 as *const i8,
        ));
        if qglUniform1f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform2f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform2fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform2f\x00" as *const u8 as *const i8,
        ));
        if qglUniform2f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform2f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform3f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform3fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform3f\x00" as *const u8 as *const i8,
        ));
        if qglUniform3f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform3f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform4f = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform4fproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform4f\x00" as *const u8 as *const i8,
        ));
        if qglUniform4f.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform4f\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1i = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1iproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1i\x00" as *const u8 as *const i8,
        ));
        if qglUniform1i.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1i\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniform1fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::Uniform1fvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniform1fv\x00" as *const u8 as *const i8,
        ));
        if qglUniform1fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniform1fv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglUniformMatrix4fv = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::UniformMatrix4fvproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glUniformMatrix4fv\x00" as *const u8 as *const i8,
        ));
        if qglUniformMatrix4fv.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glUniformMatrix4fv\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglValidateProgram = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::ValidateProgramproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glValidateProgram\x00" as *const u8 as *const i8,
        ));
        if qglValidateProgram.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glValidateProgram\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        qglVertexAttribPointer = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::VertexAttribPointerproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glVertexAttribPointer\x00" as *const u8 as *const i8,
        ));
        if qglVertexAttribPointer.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glVertexAttribPointer\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
        // error so this doesn't segfault due to NULL desktop GL functions being used
        crate::src::renderergl1::tr_subs::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Unsupported OpenGL Version: %s\n\x00" as *const u8 as *const i8,
            version,
        );
    } else {
        crate::src::renderergl1::tr_subs::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Unsupported OpenGL Version (%s), OpenGL 2.0 is required\n\x00" as *const u8
                as *const i8,
            version,
        );
    }
    if qglMajorVersion > 3
        || qglMajorVersion == 3 && qglMinorVersion >= 0
        || (qglesMajorVersion > 3
            || qglesMajorVersion == 3 && qglesMinorVersion >= 0)
    {
        qglGetStringi = ::std::mem::transmute::<
            *mut libc::c_void,
            Option<crate::qgl_h::GetStringiproc>,
        >(crate::stdlib::SDL_GL_GetProcAddress(
            b"glGetStringi\x00" as *const u8 as *const i8,
        ));
        if qglGetStringi.is_none() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ERROR: Missing OpenGL function %s\n\x00" as *const u8 as *const i8,
                b"glGetStringi\x00" as *const u8 as *const i8,
            );
            success = crate::src::qcommon::q_shared::qfalse
        }
    }
    return success;
}
/*
===============
GLimp_ClearProcAddresses

Clear addresses for OpenGL functions.
===============
*/

unsafe extern "C" fn GLimp_ClearProcAddresses() {
    qglMajorVersion = 0;
    qglMinorVersion = 0;
    qglesMajorVersion = 0;
    qglesMinorVersion = 0;
    qglBindTexture = None;
    qglBlendFunc = None;
    qglClearColor = None;
    qglClear = None;
    qglClearStencil = None;
    qglColorMask = None;
    qglCopyTexSubImage2D = None;
    qglCullFace = None;
    qglDeleteTextures = None;
    qglDepthFunc = None;
    qglDepthMask = None;
    qglDisable = None;
    qglDrawArrays = None;
    qglDrawElements = None;
    qglEnable = None;
    qglFinish = None;
    qglFlush = None;
    qglGenTextures = None;
    qglGetBooleanv = None;
    qglGetError = None;
    qglGetIntegerv = None;
    qglGetString = None;
    qglLineWidth = None;
    qglPolygonOffset = None;
    qglReadPixels = None;
    qglScissor = None;
    qglStencilFunc = None;
    qglStencilMask = None;
    qglStencilOp = None;
    qglTexImage2D = None;
    qglTexParameterf = None;
    qglTexParameteri = None;
    qglTexSubImage2D = None;
    qglTranslatef = None;
    qglViewport = None;
    qglAlphaFunc = None;
    qglColor4f = None;
    qglColorPointer = None;
    qglDisableClientState = None;
    qglEnableClientState = None;
    qglLoadIdentity = None;
    qglLoadMatrixf = None;
    qglMatrixMode = None;
    qglPopMatrix = None;
    qglPushMatrix = None;
    qglShadeModel = None;
    qglTexCoordPointer = None;
    qglTexEnvf = None;
    qglVertexPointer = None;
    qglClearDepth = None;
    qglDepthRange = None;
    qglDrawBuffer = None;
    qglPolygonMode = None;
    qglArrayElement = None;
    qglBegin = None;
    qglClipPlane = None;
    qglColor3f = None;
    qglColor4ubv = None;
    qglEnd = None;
    qglFrustum = None;
    qglOrtho = None;
    qglTexCoord2f = None;
    qglTexCoord2fv = None;
    qglVertex2f = None;
    qglVertex3f = None;
    qglVertex3fv = None;
    qglClearDepthf = None;
    qglDepthRangef = None;
    qglClipPlanef = None;
    qglFrustumf = None;
    qglOrthof = None;
    qglActiveTexture = None;
    qglCompressedTexImage2D = None;
    qglCompressedTexSubImage2D = None;
    qglBindBuffer = None;
    qglDeleteBuffers = None;
    qglGenBuffers = None;
    qglBufferData = None;
    qglBufferSubData = None;
    qglAttachShader = None;
    qglBindAttribLocation = None;
    qglCompileShader = None;
    qglCreateProgram = None;
    qglCreateShader = None;
    qglDeleteProgram = None;
    qglDeleteShader = None;
    qglDetachShader = None;
    qglDisableVertexAttribArray = None;
    qglEnableVertexAttribArray = None;
    qglGetActiveUniform = None;
    qglGetProgramiv = None;
    qglGetProgramInfoLog = None;
    qglGetShaderiv = None;
    qglGetShaderInfoLog = None;
    qglGetShaderSource = None;
    qglGetUniformLocation = None;
    qglLinkProgram = None;
    qglShaderSource = None;
    qglUseProgram = None;
    qglUniform1f = None;
    qglUniform2f = None;
    qglUniform3f = None;
    qglUniform4f = None;
    qglUniform1i = None;
    qglUniform1fv = None;
    qglUniformMatrix4fv = None;
    qglValidateProgram = None;
    qglVertexAttribPointer = None;
    qglGetStringi = None;
    qglGenQueries = None;
    qglDeleteQueries = None;
    qglBeginQuery = None;
    qglEndQuery = None;
    qglGetQueryObjectiv = None;
    qglGetQueryObjectuiv = None;
    qglBindRenderbuffer = None;
    qglDeleteRenderbuffers = None;
    qglGenRenderbuffers = None;
    qglRenderbufferStorage = None;
    qglBindFramebuffer = None;
    qglDeleteFramebuffers = None;
    qglGenFramebuffers = None;
    qglCheckFramebufferStatus = None;
    qglFramebufferTexture2D = None;
    qglFramebufferRenderbuffer = None;
    qglGenerateMipmap = None;
    qglBlitFramebuffer = None;
    qglRenderbufferStorageMultisample = None;
    qglBindVertexArray = None;
    qglDeleteVertexArrays = None;
    qglGenVertexArrays = None;
    qglBindMultiTextureEXT = None;
    qglTextureParameterfEXT = None;
    qglTextureParameteriEXT = None;
    qglTextureImage2DEXT = None;
    qglTextureSubImage2DEXT = None;
    qglCopyTextureSubImage2DEXT = None;
    qglCompressedTextureImage2DEXT = None;
    qglCompressedTextureSubImage2DEXT = None;
    qglGenerateTextureMipmapEXT = None;
    qglProgramUniform1iEXT = None;
    qglProgramUniform1fEXT = None;
    qglProgramUniform2fEXT = None;
    qglProgramUniform3fEXT = None;
    qglProgramUniform4fEXT = None;
    qglProgramUniform1fvEXT = None;
    qglProgramUniformMatrix4fvEXT = None;
    qglNamedRenderbufferStorageEXT = None;
    qglNamedRenderbufferStorageMultisampleEXT = None;
    qglCheckNamedFramebufferStatusEXT = None;
    qglNamedFramebufferTexture2DEXT = None;
    qglNamedFramebufferRenderbufferEXT = None;
    qglActiveTextureARB = None;
    qglClientActiveTextureARB = None;
    qglMultiTexCoord2fARB = None;
    qglLockArraysEXT = None;
    qglUnlockArraysEXT = None;
}
/*
===============
GLimp_SetMode
===============
*/

unsafe extern "C" fn GLimp_SetMode(
    mut mode: i32,
    mut fullscreen: crate::src::qcommon::q_shared::qboolean,
    mut noborder: crate::src::qcommon::q_shared::qboolean,
    mut fixedFunction: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut glstring: *const i8 = 0 as *const i8;
    let mut perChannelColorBits: i32 = 0;
    let mut colorBits: i32 = 0;
    let mut depthBits: i32 = 0;
    let mut stencilBits: i32 = 0;
    let mut samples: i32 = 0;
    let mut i: i32 = 0;
    let mut icon: *mut crate::stdlib::SDL_Surface = 0 as *mut crate::stdlib::SDL_Surface;
    let mut flags: crate::stdlib::Uint32 = (crate::stdlib::SDL_WINDOW_SHOWN as i32
        | crate::stdlib::SDL_WINDOW_OPENGL as i32)
        as crate::stdlib::Uint32;
    let mut desktopMode: crate::stdlib::SDL_DisplayMode = crate::stdlib::SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: 0 as *mut libc::c_void,
    };
    let mut display: i32 = 0;
    let mut x: i32 =
        (0x1fff0000u32 | 0) as i32;
    let mut y: i32 =
        (0x1fff0000u32 | 0) as i32;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"Initializing OpenGL display\n\x00" as *const u8 as *const i8,
    );
    if (*r_allowResize).integer != 0 {
        flags |=  crate::stdlib::SDL_WINDOW_RESIZABLE
    }
    icon = crate::stdlib::SDL_CreateRGBSurfaceFrom(
        CLIENT_WINDOW_ICON.pixel_data.as_ptr() as *mut libc::c_void,
        CLIENT_WINDOW_ICON.width as i32,
        CLIENT_WINDOW_ICON.height as i32,
        CLIENT_WINDOW_ICON
            .bytes_per_pixel
            .wrapping_mul(8u32) as i32,
        CLIENT_WINDOW_ICON
            .bytes_per_pixel
            .wrapping_mul(CLIENT_WINDOW_ICON.width) as i32,
        0xff,
        0xff00,
        0xff0000,
        0xff000000,
    );
    // If a window exists, note its display index
    if !SDL_window.is_null() {
        display = crate::stdlib::SDL_GetWindowDisplayIndex(SDL_window);
        if display < 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                b"SDL_GetWindowDisplayIndex() failed: %s\n\x00" as *const u8 as *const i8,
                crate::stdlib::SDL_GetError(),
            );
        }
    }
    if display >= 0
        && crate::stdlib::SDL_GetDesktopDisplayMode(display, &mut desktopMode) == 0
    {
        crate::src::renderergl1::tr_init::displayAspect =
            desktopMode.w as f32 / desktopMode.h as f32;
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Display aspect: %.3f\n\x00" as *const u8 as *const i8,
            crate::src::renderergl1::tr_init::displayAspect as f64,
        );
    } else {
        crate::stdlib::memset(
            &mut desktopMode as *mut crate::stdlib::SDL_DisplayMode as *mut libc::c_void,
            0,
            
            ::std::mem::size_of::<crate::stdlib::SDL_DisplayMode>(),
        );
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Cannot determine display aspect, assuming 1.333\n\x00" as *const u8
                as *const i8,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"...setting mode %d:\x00" as *const u8 as *const i8,
        mode,
    );
    if mode == -(2) {
        // use desktop video resolution
        if desktopMode.h > 0 {
            crate::src::renderergl1::tr_init::glConfig.vidWidth = desktopMode.w;
            crate::src::renderergl1::tr_init::glConfig.vidHeight = desktopMode.h
        } else {
            crate::src::renderergl1::tr_init::glConfig.vidWidth = 640;
            crate::src::renderergl1::tr_init::glConfig.vidHeight = 480;
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"Cannot determine display resolution, assuming 640x480\n\x00" as *const u8
                    as *const i8,
            );
        }
        crate::src::renderergl1::tr_init::glConfig.windowAspect =
            crate::src::renderergl1::tr_init::glConfig.vidWidth as f32
                / crate::src::renderergl1::tr_init::glConfig.vidHeight as f32
    } else if crate::src::renderergl1::tr_init::R_GetModeInfo(
        &mut crate::src::renderergl1::tr_init::glConfig.vidWidth,
        &mut crate::src::renderergl1::tr_init::glConfig.vidHeight,
        &mut crate::src::renderergl1::tr_init::glConfig.windowAspect,
        mode,
    ) as u64
        == 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b" invalid mode\n\x00" as *const u8 as *const i8,
        );
        return RSERR_INVALID_MODE as i32;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b" %d %d\n\x00" as *const u8 as *const i8,
        crate::src::renderergl1::tr_init::glConfig.vidWidth,
        crate::src::renderergl1::tr_init::glConfig.vidHeight,
    );
    // Center window
    if (*r_centerWindow).integer != 0 && fullscreen as u64 == 0 {
        x = desktopMode.w / 2
            - crate::src::renderergl1::tr_init::glConfig.vidWidth / 2;
        y = desktopMode.h / 2
            - crate::src::renderergl1::tr_init::glConfig.vidHeight / 2
    }
    // Destroy existing state if it exists
    if !SDL_glContext.is_null() {
        GLimp_ClearProcAddresses();
        crate::stdlib::SDL_GL_DeleteContext(SDL_glContext);
        SDL_glContext = 0 as *mut libc::c_void
    }
    if !SDL_window.is_null() {
        crate::stdlib::SDL_GetWindowPosition(SDL_window, &mut x, &mut y);
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
            b"Existing window at %dx%d before being destroyed\n\x00" as *const u8
                as *const i8,
            x,
            y,
        );
        crate::stdlib::SDL_DestroyWindow(SDL_window);
        SDL_window = 0 as *mut crate::stdlib::SDL_Window
    }
    if fullscreen as u64 != 0 {
        flags |=  crate::stdlib::SDL_WINDOW_FULLSCREEN;
        crate::src::renderergl1::tr_init::glConfig.isFullscreen =
            crate::src::qcommon::q_shared::qtrue
    } else {
        if noborder as u64 != 0 {
            flags |=  crate::stdlib::SDL_WINDOW_BORDERLESS
        }
        crate::src::renderergl1::tr_init::glConfig.isFullscreen =
            crate::src::qcommon::q_shared::qfalse
    }
    colorBits = (*crate::src::renderergl1::tr_init::r_colorbits).value as i32;
    if colorBits == 0 || colorBits >= 32 {
        colorBits = 24
    }
    if (*crate::src::renderergl1::tr_init::r_depthbits).value == 0. {
        depthBits = 24
    } else {
        depthBits = (*crate::src::renderergl1::tr_init::r_depthbits).value as i32
    }
    stencilBits = (*crate::src::renderergl1::tr_init::r_stencilbits).value as i32;
    samples = (*crate::src::renderergl1::tr_init::r_ext_multisample).value as i32;
    let mut current_block_184: u64;
    i = 0;
    while i < 16 {
        let mut testColorBits: i32 = 0;
        let mut testDepthBits: i32 = 0;
        let mut testStencilBits: i32 = 0;
        let mut realColorBits: [i32; 3] = [0; 3];
        // 0 - default
        // 1 - minus colorBits
        // 2 - minus depthBits
        // 3 - minus stencil
        if i % 4 == 0 && i != 0 {
            let mut current_block_75: u64;
            // one pass, reduce
            match i / 4 {
                2 => {
                    if colorBits == 24 {
                        colorBits = 16
                    }
                    current_block_75 = 10778260831612459202;
                }
                1 => {
                    if depthBits == 24 {
                        depthBits = 16
                    } else if depthBits == 16 {
                        depthBits = 8
                    }
                    current_block_75 = 13735627286979930116;
                }
                3 => {
                    current_block_75 = 13735627286979930116;
                }
                _ => {
                    current_block_75 = 10778260831612459202;
                }
            }
            match current_block_75 {
                13735627286979930116 => {
                    if stencilBits == 24 {
                        stencilBits = 16
                    } else if stencilBits == 16 {
                        stencilBits = 8
                    }
                }
                _ => {}
            }
        }
        testColorBits = colorBits;
        testDepthBits = depthBits;
        testStencilBits = stencilBits;
        if i % 4 == 3 {
            // reduce colorBits
            if testColorBits == 24 {
                testColorBits = 16
            }
        }
        if i % 4 == 2 {
            // reduce depthBits
            if testDepthBits == 24 {
                testDepthBits = 16
            } else if testDepthBits == 16 {
                testDepthBits = 8
            }
        }
        if i % 4 == 1 {
            // reduce stencilBits
            if testStencilBits == 24 {
                testStencilBits = 16
            } else if testStencilBits == 16 {
                testStencilBits = 8
            } else {
                testStencilBits = 0
            }
        }
        if testColorBits == 24 {
            perChannelColorBits = 8
        } else {
            perChannelColorBits = 4
        }
        /* Fix for SGIs grabbing too many bits of color */
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_RED_SIZE, perChannelColorBits);
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_GREEN_SIZE, perChannelColorBits);
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_BLUE_SIZE, perChannelColorBits);
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_DEPTH_SIZE, testDepthBits);
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_STENCIL_SIZE, testStencilBits);
        crate::stdlib::SDL_GL_SetAttribute(
            crate::stdlib::SDL_GL_MULTISAMPLEBUFFERS,
            if samples != 0 {
                1
            } else {
                0
            },
        );
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_MULTISAMPLESAMPLES, samples);
        if (*crate::src::renderergl1::tr_init::r_stereoEnabled).integer != 0 {
            crate::src::renderergl1::tr_init::glConfig.stereoEnabled =
                crate::src::qcommon::q_shared::qtrue;
            crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_STEREO, 1i32);
        } else {
            crate::src::renderergl1::tr_init::glConfig.stereoEnabled =
                crate::src::qcommon::q_shared::qfalse;
            crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_STEREO, 0i32);
        }
        crate::stdlib::SDL_GL_SetAttribute(crate::stdlib::SDL_GL_DOUBLEBUFFER, 1);
        // if multisampling is enabled on X11, this causes create window to fail.
        SDL_window = crate::stdlib::SDL_CreateWindow(
            b"ioquake3\x00" as *const u8 as *const i8,
            x,
            y,
            crate::src::renderergl1::tr_init::glConfig.vidWidth,
            crate::src::renderergl1::tr_init::glConfig.vidHeight,
            flags,
        );
        if SDL_window.is_null() {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                b"SDL_CreateWindow failed: %s\n\x00" as *const u8 as *const i8,
                crate::stdlib::SDL_GetError(),
            );
        } else {
            if fullscreen as u64 != 0 {
                let mut mode_0: crate::stdlib::SDL_DisplayMode = crate::stdlib::SDL_DisplayMode {
                    format: 0,
                    w: 0,
                    h: 0,
                    refresh_rate: 0,
                    driverdata: 0 as *mut libc::c_void,
                };
                match testColorBits {
                    16 => {
                        mode_0.format =  crate::stdlib::SDL_PIXELFORMAT_RGB565;
                        current_block_184 = 1425453989644512380;
                    }
                    24 => {
                        mode_0.format =  crate::stdlib::SDL_PIXELFORMAT_RGB24;
                        current_block_184 = 1425453989644512380;
                    }
                    _ => {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                            b"testColorBits is %d, can\'t fullscreen\n\x00" as *const u8
                                as *const i8,
                            testColorBits,
                        );
                        current_block_184 = 5597585068398118923;
                    }
                }
                match current_block_184 {
                    5597585068398118923 => {}
                    _ => {
                        mode_0.w = crate::src::renderergl1::tr_init::glConfig.vidWidth;
                        mode_0.h = crate::src::renderergl1::tr_init::glConfig.vidHeight;
                        crate::src::renderergl1::tr_init::glConfig.displayFrequency =
                            crate::src::renderergl1::tr_main::ri
                                .Cvar_VariableIntegerValue
                                .expect("non-null function pointer")(
                                b"r_displayRefresh\x00" as *const u8 as *const i8,
                            );
                        mode_0.refresh_rate =
                            crate::src::renderergl1::tr_init::glConfig.displayFrequency;
                        mode_0.driverdata = 0 as *mut libc::c_void;
                        if crate::stdlib::SDL_SetWindowDisplayMode(SDL_window, &mut mode_0)
                            < 0
                        {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                                b"SDL_SetWindowDisplayMode failed: %s\n\x00" as *const u8
                                    as *const i8,
                                crate::stdlib::SDL_GetError(),
                            );
                            current_block_184 = 5597585068398118923;
                        } else {
                            current_block_184 = 2872334340672008580;
                        }
                    }
                }
            } else {
                current_block_184 = 2872334340672008580;
            }
            match current_block_184 {
                5597585068398118923 => {}
                _ => {
                    crate::stdlib::SDL_SetWindowIcon(SDL_window, icon);
                    if fixedFunction as u64 == 0 {
                        let mut profileMask: i32 = 0;
                        let mut majorVersion: i32 = 0;
                        let mut minorVersion: i32 = 0;
                        crate::stdlib::SDL_GL_GetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK,
                            &mut profileMask,
                        );
                        crate::stdlib::SDL_GL_GetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION,
                            &mut majorVersion,
                        );
                        crate::stdlib::SDL_GL_GetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION,
                            &mut minorVersion,
                        );
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_ALL as i32,
                            b"Trying to get an OpenGL 3.2 core context\n\x00" as *const u8
                                as *const i8,
                        );
                        crate::stdlib::SDL_GL_SetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK,
                            crate::stdlib::SDL_GL_CONTEXT_PROFILE_CORE as i32,
                        );
                        crate::stdlib::SDL_GL_SetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION,
                            3,
                        );
                        crate::stdlib::SDL_GL_SetAttribute(
                            crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION,
                            2,
                        );
                        SDL_glContext = crate::stdlib::SDL_GL_CreateContext(SDL_window);
                        if SDL_glContext.is_null() {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                b"SDL_GL_CreateContext failed: %s\n\x00" as *const u8
                                    as *const i8,
                                crate::stdlib::SDL_GetError(),
                            );
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                b"Reverting to default context\n\x00" as *const u8
                                    as *const i8,
                            );
                            crate::stdlib::SDL_GL_SetAttribute(
                                crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK,
                                profileMask,
                            );
                            crate::stdlib::SDL_GL_SetAttribute(
                                crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION,
                                majorVersion,
                            );
                            crate::stdlib::SDL_GL_SetAttribute(
                                crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION,
                                minorVersion,
                            );
                        } else {
                            let mut renderer: *const i8 = 0 as *const i8;
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                b"SDL_GL_CreateContext succeeded.\n\x00" as *const u8
                                    as *const i8,
                            );
                            if GLimp_GetProcAddresses(fixedFunction) as u64 != 0 {
                                renderer = qglGetString.expect("non-null function pointer")(
                                    0x1f01u32,
                                ) as *const i8
                            } else {
                                crate::src::renderergl1::tr_main::ri.Printf.expect("non-null function pointer")(crate::src::qcommon::q_shared::PRINT_ALL
                                                                                  as
                                                                                  i32,
                                                                              b"GLimp_GetProcAddresses() failed for OpenGL 3.2 core context\n\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const i8);
                                renderer = 0 as *const i8
                            }
                            if renderer.is_null()
                                || (!crate::stdlib::strstr(
                                    renderer,
                                    b"Software Renderer\x00" as *const u8 as *const i8,
                                )
                                .is_null()
                                    || !crate::stdlib::strstr(
                                        renderer,
                                        b"Software Rasterizer\x00" as *const u8
                                            as *const i8,
                                    )
                                    .is_null())
                            {
                                if !renderer.is_null() {
                                    crate::src::renderergl1::tr_main::ri
                                        .Printf
                                        .expect("non-null function pointer")(
                                        crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                        b"GL_RENDERER is %s, rejecting context\n\x00" as *const u8
                                            as *const i8,
                                        renderer,
                                    );
                                }
                                GLimp_ClearProcAddresses();
                                crate::stdlib::SDL_GL_DeleteContext(SDL_glContext);
                                SDL_glContext = 0 as *mut libc::c_void;
                                crate::stdlib::SDL_GL_SetAttribute(
                                    crate::stdlib::SDL_GL_CONTEXT_PROFILE_MASK,
                                    profileMask,
                                );
                                crate::stdlib::SDL_GL_SetAttribute(
                                    crate::stdlib::SDL_GL_CONTEXT_MAJOR_VERSION,
                                    majorVersion,
                                );
                                crate::stdlib::SDL_GL_SetAttribute(
                                    crate::stdlib::SDL_GL_CONTEXT_MINOR_VERSION,
                                    minorVersion,
                                );
                            }
                        }
                    } else {
                        SDL_glContext = 0 as *mut libc::c_void
                    }
                    if SDL_glContext.is_null() {
                        SDL_glContext = crate::stdlib::SDL_GL_CreateContext(SDL_window);
                        if SDL_glContext.is_null() {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                                b"SDL_GL_CreateContext failed: %s\n\x00" as *const u8
                                    as *const i8,
                                crate::stdlib::SDL_GetError(),
                            );
                            crate::stdlib::SDL_DestroyWindow(SDL_window);
                            SDL_window = 0 as *mut crate::stdlib::SDL_Window;
                            current_block_184 = 5597585068398118923;
                        } else if GLimp_GetProcAddresses(fixedFunction) as u64 == 0 {
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                b"GLimp_GetProcAddresses() failed\n\x00" as *const u8
                                    as *const i8,
                            );
                            GLimp_ClearProcAddresses();
                            crate::stdlib::SDL_GL_DeleteContext(SDL_glContext);
                            SDL_glContext = 0 as *mut libc::c_void;
                            crate::stdlib::SDL_DestroyWindow(SDL_window);
                            SDL_window = 0 as *mut crate::stdlib::SDL_Window;
                            current_block_184 = 5597585068398118923;
                        } else {
                            current_block_184 = 1953367063549441504;
                        }
                    } else {
                        current_block_184 = 1953367063549441504;
                    }
                    match current_block_184 {
                        5597585068398118923 => {}
                        _ => {
                            qglClearColor.expect("non-null function pointer")(
                                0f32,
                                0f32,
                                0f32,
                                1f32,
                            );
                            qglClear.expect("non-null function pointer")(
                                0x4000u32,
                            );
                            crate::stdlib::SDL_GL_SwapWindow(SDL_window);
                            if crate::stdlib::SDL_GL_SetSwapInterval(
                                (*crate::src::renderergl1::tr_init::r_swapInterval).integer,
                            ) == -(1)
                            {
                                crate::src::renderergl1::tr_main::ri
                                    .Printf
                                    .expect("non-null function pointer")(
                                    crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                                    b"SDL_GL_SetSwapInterval failed: %s\n\x00" as *const u8
                                        as *const i8,
                                    crate::stdlib::SDL_GetError(),
                                );
                            }
                            crate::stdlib::SDL_GL_GetAttribute(
                                crate::stdlib::SDL_GL_RED_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(0),
                            );
                            crate::stdlib::SDL_GL_GetAttribute(
                                crate::stdlib::SDL_GL_GREEN_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(1),
                            );
                            crate::stdlib::SDL_GL_GetAttribute(
                                crate::stdlib::SDL_GL_BLUE_SIZE,
                                &mut *realColorBits.as_mut_ptr().offset(2),
                            );
                            crate::stdlib::SDL_GL_GetAttribute(
                                crate::stdlib::SDL_GL_DEPTH_SIZE,
                                &mut crate::src::renderergl1::tr_init::glConfig.depthBits,
                            );
                            crate::stdlib::SDL_GL_GetAttribute(
                                crate::stdlib::SDL_GL_STENCIL_SIZE,
                                &mut crate::src::renderergl1::tr_init::glConfig.stencilBits,
                            );
                            crate::src::renderergl1::tr_init::glConfig.colorBits = realColorBits
                                [0]
                                + realColorBits[1]
                                + realColorBits[2];
                            crate::src::renderergl1::tr_main::ri
                                .Printf
                                .expect("non-null function pointer")(
                                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                                b"Using %d color bits, %d depth, %d stencil display.\n\x00"
                                    as *const u8
                                    as *const i8,
                                crate::src::renderergl1::tr_init::glConfig.colorBits,
                                crate::src::renderergl1::tr_init::glConfig.depthBits,
                                crate::src::renderergl1::tr_init::glConfig.stencilBits,
                            );
                            break;
                        }
                    }
                }
            }
        }
        i += 1
    }
    crate::stdlib::SDL_FreeSurface(icon);
    if SDL_window.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Couldn\'t get a visual\n\x00" as *const u8 as *const i8,
        );
        return RSERR_INVALID_MODE as i32;
    }
    GLimp_DetectAvailableModes();
    glstring = qglGetString.expect("non-null function pointer")(
        0x1f01u32,
    ) as *mut i8;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_RENDERER: %s\n\x00" as *const u8 as *const i8,
        glstring,
    );
    return RSERR_OK as i32;
}
/*
===============
GLimp_StartDriverAndSetMode
===============
*/

unsafe extern "C" fn GLimp_StartDriverAndSetMode(
    mut mode: i32,
    mut fullscreen: crate::src::qcommon::q_shared::qboolean,
    mut noborder: crate::src::qcommon::q_shared::qboolean,
    mut gl3Core: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut err: rserr_t = RSERR_OK;
    if crate::stdlib::SDL_WasInit(0x20) == 0 {
        let mut driverName: *const i8 = 0 as *const i8;
        if crate::stdlib::SDL_Init(0x20) != 0 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"SDL_Init( SDL_INIT_VIDEO ) FAILED (%s)\n\x00" as *const u8 as *const i8,
                crate::stdlib::SDL_GetError(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        driverName = crate::stdlib::SDL_GetCurrentVideoDriver();
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"SDL using driver \"%s\"\n\x00" as *const u8 as *const i8,
            driverName,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_sdlDriver\x00" as *const u8 as *const i8,
            driverName,
        );
    }
    if  fullscreen != 0
        && crate::src::renderergl1::tr_main::ri
            .Cvar_VariableIntegerValue
            .expect("non-null function pointer")(
            b"in_nograb\x00" as *const u8 as *const i8,
        ) != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Fullscreen not allowed with in_nograb 1\n\x00" as *const u8 as *const i8,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_fullscreen\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        (*crate::src::renderergl1::tr_init::r_fullscreen).modified =
            crate::src::qcommon::q_shared::qfalse;
        fullscreen = crate::src::qcommon::q_shared::qfalse
    }
    err = GLimp_SetMode(mode, fullscreen, noborder, gl3Core) as rserr_t;
    match  err {
        1 => {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...WARNING: fullscreen unavailable in this mode\n\x00" as *const u8
                    as *const i8,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        2 => {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...WARNING: could not set the given mode (%d)\n\x00" as *const u8
                    as *const i8,
                mode,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        _ => {}
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
GLimp_InitExtensions
===============
*/

unsafe extern "C" fn GLimp_InitExtensions(
    mut fixedFunction: crate::src::qcommon::q_shared::qboolean,
) {
    if (*crate::src::renderergl1::tr_init::r_allowExtensions).integer == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"* IGNORING OPENGL EXTENSIONS *\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"Initializing OpenGL extensions\n\x00" as *const u8 as *const i8,
    );
    crate::src::renderergl1::tr_init::glConfig.textureCompression = crate::tr_types_h::TC_NONE;
    // GL_EXT_texture_compression_s3tc
    if  crate::stdlib::SDL_GL_ExtensionSupported(
        b"GL_ARB_texture_compression\x00" as *const u8 as *const i8,
    )
        != 0
        &&  crate::stdlib::SDL_GL_ExtensionSupported(
            b"GL_EXT_texture_compression_s3tc\x00" as *const u8 as *const i8,
        )
            != 0
    {
        if (*crate::src::renderergl1::tr_init::r_ext_compressed_textures).value != 0. {
            crate::src::renderergl1::tr_init::glConfig.textureCompression =
                crate::tr_types_h::TC_S3TC_ARB;
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...using GL_EXT_texture_compression_s3tc\n\x00" as *const u8
                    as *const i8,
            );
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...ignoring GL_EXT_texture_compression_s3tc\n\x00" as *const u8
                    as *const i8,
            );
        }
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"...GL_EXT_texture_compression_s3tc not found\n\x00" as *const u8
                as *const i8,
        );
    }
    // GL_S3_s3tc ... legacy extension before GL_EXT_texture_compression_s3tc.
    if  crate::src::renderergl1::tr_init::glConfig.textureCompression
        ==  crate::tr_types_h::TC_NONE
    {
        if crate::stdlib::SDL_GL_ExtensionSupported(
            b"GL_S3_s3tc\x00" as *const u8 as *const i8,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_compressed_textures).value != 0. {
                crate::src::renderergl1::tr_init::glConfig.textureCompression =
                    crate::tr_types_h::TC_S3TC;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...using GL_S3_s3tc\n\x00" as *const u8 as *const i8,
                );
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...ignoring GL_S3_s3tc\n\x00" as *const u8 as *const i8,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...GL_S3_s3tc not found\n\x00" as *const u8 as *const i8,
            );
        }
    }
    // OpenGL 1 fixed function pipeline
    if fixedFunction as u64 != 0 {
        // GL_EXT_texture_env_add
        crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable =
            crate::src::qcommon::q_shared::qfalse;
        if crate::stdlib::SDL_GL_ExtensionSupported(
            b"GL_EXT_texture_env_add\x00" as *const u8 as *const i8,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_texture_env_add).integer != 0 {
                crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable =
                    crate::src::qcommon::q_shared::qtrue;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...using GL_EXT_texture_env_add\n\x00" as *const u8 as *const i8,
                );
            } else {
                crate::src::renderergl1::tr_init::glConfig.textureEnvAddAvailable =
                    crate::src::qcommon::q_shared::qfalse;
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...ignoring GL_EXT_texture_env_add\n\x00" as *const u8 as *const i8,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...GL_EXT_texture_env_add not found\n\x00" as *const u8 as *const i8,
            );
        }
        // GL_ARB_multitexture
        qglMultiTexCoord2fARB = None;
        qglActiveTextureARB = None;
        qglClientActiveTextureARB = None;
        if crate::stdlib::SDL_GL_ExtensionSupported(
            b"GL_ARB_multitexture\x00" as *const u8 as *const i8,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_multitexture).value != 0. {
                qglMultiTexCoord2fARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<
                        unsafe extern "C" fn(
                            _: crate::stdlib::GLenum,
                            _: crate::stdlib::GLfloat,
                            _: crate::stdlib::GLfloat,
                        ) -> (),
                    >,
                >(crate::stdlib::SDL_GL_GetProcAddress(
                    b"glMultiTexCoord2fARB\x00" as *const u8 as *const i8,
                ));
                qglActiveTextureARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ()>,
                >(crate::stdlib::SDL_GL_GetProcAddress(
                    b"glActiveTextureARB\x00" as *const u8 as *const i8,
                ));
                qglClientActiveTextureARB = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(_: crate::stdlib::GLenum) -> ()>,
                >(crate::stdlib::SDL_GL_GetProcAddress(
                    b"glClientActiveTextureARB\x00" as *const u8 as *const i8,
                ));
                if qglActiveTextureARB.is_some() {
                    let mut glint: crate::stdlib::GLint = 0;
                    qglGetIntegerv.expect("non-null function pointer")(
                        0x84e2u32,
                        &mut glint,
                    );
                    crate::src::renderergl1::tr_init::glConfig.numTextureUnits = glint;
                    if crate::src::renderergl1::tr_init::glConfig.numTextureUnits > 1
                    {
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_ALL as i32,
                            b"...using GL_ARB_multitexture\n\x00" as *const u8
                                as *const i8,
                        );
                    } else {
                        qglMultiTexCoord2fARB = None;
                        qglActiveTextureARB = None;
                        qglClientActiveTextureARB = None;
                        crate::src::renderergl1::tr_main::ri
                            .Printf
                            .expect("non-null function pointer")(
                            crate::src::qcommon::q_shared::PRINT_ALL as i32,
                            b"...not using GL_ARB_multitexture, < 2 texture units\n\x00"
                                as *const u8 as *const i8,
                        );
                    }
                }
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...ignoring GL_ARB_multitexture\n\x00" as *const u8 as *const i8,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...GL_ARB_multitexture not found\n\x00" as *const u8 as *const i8,
            );
        }
        // GL_EXT_compiled_vertex_array
        if crate::stdlib::SDL_GL_ExtensionSupported(
            b"GL_EXT_compiled_vertex_array\x00" as *const u8 as *const i8,
        ) as u64
            != 0
        {
            if (*crate::src::renderergl1::tr_init::r_ext_compiled_vertex_array).value != 0. {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...using GL_EXT_compiled_vertex_array\n\x00" as *const u8
                        as *const i8,
                );
                qglLockArraysEXT = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<
                        unsafe extern "C" fn(
                            _: crate::stdlib::GLint,
                            _: crate::stdlib::GLint,
                        ) -> (),
                    >,
                >(crate::stdlib::SDL_GL_GetProcAddress(
                    b"glLockArraysEXT\x00" as *const u8 as *const i8,
                ));
                qglUnlockArraysEXT = ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn() -> ()>,
                >(crate::stdlib::SDL_GL_GetProcAddress(
                    b"glUnlockArraysEXT\x00" as *const u8 as *const i8,
                ));
                if qglLockArraysEXT.is_none() || qglUnlockArraysEXT.is_none() {
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_FATAL as i32,
                        b"bad getprocaddress\x00" as *const u8 as *const i8,
                    );
                }
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...ignoring GL_EXT_compiled_vertex_array\n\x00" as *const u8
                        as *const i8,
                );
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...GL_EXT_compiled_vertex_array not found\n\x00" as *const u8
                    as *const i8,
            );
        }
    }
    crate::src::renderergl1::tr_init::textureFilterAnisotropic =
        crate::src::qcommon::q_shared::qfalse;
    if crate::stdlib::SDL_GL_ExtensionSupported(
        b"GL_EXT_texture_filter_anisotropic\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        if (*crate::src::renderergl1::tr_init::r_ext_texture_filter_anisotropic).integer != 0 {
            qglGetIntegerv.expect("non-null function pointer")(
                0x84ffu32,
                
                &mut crate::src::renderergl1::tr_init::maxAnisotropy
                    as *mut  i32,
            );
            if crate::src::renderergl1::tr_init::maxAnisotropy <= 0 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...GL_EXT_texture_filter_anisotropic not properly supported!\n\x00"
                        as *const u8 as *const i8,
                );
                crate::src::renderergl1::tr_init::maxAnisotropy = 0
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"...using GL_EXT_texture_filter_anisotropic (max: %i)\n\x00" as *const u8
                        as *const i8,
                    crate::src::renderergl1::tr_init::maxAnisotropy,
                );
                crate::src::renderergl1::tr_init::textureFilterAnisotropic =
                    crate::src::qcommon::q_shared::qtrue
            }
        } else {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"...ignoring GL_EXT_texture_filter_anisotropic\n\x00" as *const u8
                    as *const i8,
            );
        }
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"...GL_EXT_texture_filter_anisotropic not found\n\x00" as *const u8
                as *const i8,
        );
    };
}
// 640 * 480
/*
===============
GLimp_Init

This routine is responsible for initializing the OS specific portions
of OpenGL
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_Init(mut fixedFunction: crate::src::qcommon::q_shared::qboolean) {
    let mut current_block: u64;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
        b"Glimp_Init( )\n\x00" as *const u8 as *const i8,
    );
    r_allowSoftwareGL = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_allowSoftwareGL\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x20,
    );
    r_sdlDriver = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_sdlDriver\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x40,
    );
    r_allowResize = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_allowResize\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x1 | 0x20,
    );
    r_centerWindow = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_centerWindow\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x1 | 0x20,
    );
    if crate::src::renderergl1::tr_main::ri
        .Cvar_VariableIntegerValue
        .expect("non-null function pointer")(
        b"com_abnormalExit\x00" as *const u8 as *const i8,
    ) != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_mode\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::va(
                
                b"%d\x00" as *const  u8 as *mut i8,
                3i32,
            ),
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_fullscreen\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_centerWindow\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"com_abnormalExit\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .Sys_GLimpInit
        .expect("non-null function pointer")();
    // Create the window and set up the context
    if !(GLimp_StartDriverAndSetMode(
        (*crate::src::renderergl1::tr_init::r_mode).integer,
        (*crate::src::renderergl1::tr_init::r_fullscreen).integer
            as crate::src::qcommon::q_shared::qboolean,
        (*crate::src::renderergl1::tr_init::r_noborder).integer
            as crate::src::qcommon::q_shared::qboolean,
        fixedFunction,
    ) as u64
        != 0)
    {
        // Try again, this time in a platform specific "safe mode"
        crate::src::renderergl1::tr_main::ri
            .Sys_GLimpSafeInit
            .expect("non-null function pointer")();
        if !(GLimp_StartDriverAndSetMode(
            (*crate::src::renderergl1::tr_init::r_mode).integer,
            (*crate::src::renderergl1::tr_init::r_fullscreen).integer
                as crate::src::qcommon::q_shared::qboolean,
            crate::src::qcommon::q_shared::qfalse,
            fixedFunction,
        ) as u64
            != 0)
        {
            // Finally, try the default screen resolution
            if (*crate::src::renderergl1::tr_init::r_mode).integer != 3 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_ALL as i32,
                    b"Setting r_mode %d failed, falling back on r_mode %d\n\x00" as *const u8
                        as *const i8,
                    (*crate::src::renderergl1::tr_init::r_mode).integer,
                    3i32,
                );
                if GLimp_StartDriverAndSetMode(
                    3,
                    crate::src::qcommon::q_shared::qfalse,
                    crate::src::qcommon::q_shared::qfalse,
                    fixedFunction,
                ) as u64
                    != 0
                {
                    current_block = 18052364713975350134;
                } else {
                    current_block = 7149356873433890176;
                }
            } else {
                current_block = 7149356873433890176;
            }
            match current_block {
                18052364713975350134 => {}
                _ => {
                    // Nothing worked, give up
                    crate::src::renderergl1::tr_main::ri
                        .Error
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::ERR_FATAL as i32,
                        b"GLimp_Init() - could not load OpenGL subsystem\x00" as *const u8
                            as *const i8,
                    );
                }
            }
        }
    }
    // These values force the UI to disable driver selection
    crate::src::renderergl1::tr_init::glConfig.driverType = crate::tr_types_h::GLDRV_ICD;
    crate::src::renderergl1::tr_init::glConfig.hardwareType = crate::tr_types_h::GLHW_GENERIC;
    // Only using SDL_SetWindowBrightness to determine if hardware gamma is supported
    crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma =
        ((((*crate::src::renderergl1::tr_init::r_ignorehwgamma).integer == 0
            && crate::stdlib::SDL_SetWindowBrightness(SDL_window, 1.0) >= 0))) as crate::src::qcommon::q_shared::qboolean;
    // get our config strings
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .vendor_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(
            0x1f00u32,
        ) as *mut i8,
        
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .renderer_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(
            0x1f01u32,
        ) as *mut i8,
        
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if *crate::src::renderergl1::tr_init::glConfig
        .renderer_string
        .as_mut_ptr() as i32
        != 0
        && crate::src::renderergl1::tr_init::glConfig.renderer_string[crate::stdlib::strlen(
            crate::src::renderergl1::tr_init::glConfig
                .renderer_string
                .as_mut_ptr(),
        )
        .wrapping_sub(1usize)] as i32
            == '\n' as i32
    {
        crate::src::renderergl1::tr_init::glConfig.renderer_string[crate::stdlib::strlen(
            crate::src::renderergl1::tr_init::glConfig
                .renderer_string
                .as_mut_ptr(),
        )
        .wrapping_sub(1usize)] = 0
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::renderergl1::tr_init::glConfig
            .version_string
            .as_mut_ptr(),
        qglGetString.expect("non-null function pointer")(
            0x1f02u32,
        ) as *mut i8,
        
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    // manually create extension list if using OpenGL 3
    if qglGetStringi.is_some() {
        let mut i: i32 = 0;
        let mut numExtensions: i32 = 0;
        let mut extensionLength: i32 = 0;
        let mut listLength: i32 = 0;
        let mut extension: *const i8 = 0 as *const i8;
        qglGetIntegerv.expect("non-null function pointer")(
            0x821du32,
            &mut numExtensions,
        );
        listLength = 0;
        i = 0;
        while i < numExtensions {
            extension = qglGetStringi.expect("non-null function pointer")(
                0x1f03,
                i as crate::stdlib::GLuint,
            ) as *mut i8;
            extensionLength = crate::stdlib::strlen(extension) as i32;
            if (listLength + extensionLength + 1) as usize
                >=  ::std::mem::size_of::<[i8; 8192]>()
            {
                break;
            }
            if i > 0 {
                crate::src::qcommon::q_shared::Q_strcat(
                    crate::src::renderergl1::tr_init::glConfig
                        .extensions_string
                        .as_mut_ptr(),
                    
                    ::std::mem::size_of::<[i8; 8192]>() as i32,
                    b" \x00" as *const u8 as *const i8,
                );
                listLength += 1
            }
            crate::src::qcommon::q_shared::Q_strcat(
                crate::src::renderergl1::tr_init::glConfig
                    .extensions_string
                    .as_mut_ptr(),
                
                ::std::mem::size_of::<[i8; 8192]>() as i32,
                extension,
            );
            listLength += extensionLength;
            i += 1
        }
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            crate::src::renderergl1::tr_init::glConfig
                .extensions_string
                .as_mut_ptr(),
            qglGetString.expect("non-null function pointer")(
                0x1f03u32,
            ) as *mut i8,
            
            ::std::mem::size_of::<[i8; 8192]>() as i32,
        );
    }
    // initialize extensions
    GLimp_InitExtensions(fixedFunction);
    crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_availableModes\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x40,
    );
    // This depends on SDL_INIT_VIDEO, hence having it here
    crate::src::renderergl1::tr_main::ri
        .IN_Init
        .expect("non-null function pointer")(SDL_window as *mut libc::c_void);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
// font stuff
/*
=============================================================

IMAGE LOADERS

=============================================================
*/
/*
====================================================================

IMPLEMENTATION SPECIFIC FUNCTIONS

====================================================================
*/
/*
===============
GLimp_EndFrame

Responsible for doing a swapbuffers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GLimp_EndFrame() {
    // don't flip if drawing to front buffer
    if crate::src::qcommon::q_shared::Q_stricmp(
        (*crate::src::renderergl1::tr_init::r_drawBuffer).string,
        b"GL_FRONT\x00" as *const u8 as *const i8,
    ) != 0
    {
        crate::stdlib::SDL_GL_SwapWindow(SDL_window);
    }
    if (*crate::src::renderergl1::tr_init::r_fullscreen).modified as u64 != 0 {
        let mut fullscreen: i32 = 0;
        let mut needToToggle: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        let mut sdlToggled: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        // Find out the current state
        fullscreen = (crate::stdlib::SDL_GetWindowFlags(SDL_window)
            &  crate::stdlib::SDL_WINDOW_FULLSCREEN
            != 0) as i32;
        if (*crate::src::renderergl1::tr_init::r_fullscreen).integer != 0
            && crate::src::renderergl1::tr_main::ri
                .Cvar_VariableIntegerValue
                .expect("non-null function pointer")(
                b"in_nograb\x00" as *const u8 as *const i8,
            ) != 0
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"Fullscreen not allowed with in_nograb 1\n\x00" as *const u8
                    as *const i8,
            );
            crate::src::renderergl1::tr_main::ri
                .Cvar_Set
                .expect("non-null function pointer")(
                b"r_fullscreen\x00" as *const u8 as *const i8,
                b"0\x00" as *const u8 as *const i8,
            );
            (*crate::src::renderergl1::tr_init::r_fullscreen).modified =
                crate::src::qcommon::q_shared::qfalse
        }
        // Is the state we want different from the current state?
        needToToggle = (((((*crate::src::renderergl1::tr_init::r_fullscreen).integer != 0)
            as i32
            != fullscreen)))
            as crate::src::qcommon::q_shared::qboolean;
        if needToToggle as u64 != 0 {
            sdlToggled = (((crate::stdlib::SDL_SetWindowFullscreen(
                SDL_window,
                (*crate::src::renderergl1::tr_init::r_fullscreen).integer as crate::stdlib::Uint32,
            ) >= 0)))
                as crate::src::qcommon::q_shared::qboolean;
            // SDL_WM_ToggleFullScreen didn't work, so do it the slow way
            if sdlToggled as u64 == 0 {
                crate::src::renderergl1::tr_main::ri
                    .Cmd_ExecuteText
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                    b"vid_restart\n\x00" as *const u8 as *const i8,
                );
            }
            crate::src::renderergl1::tr_main::ri
                .IN_Restart
                .expect("non-null function pointer")();
        }
        (*crate::src::renderergl1::tr_init::r_fullscreen).modified =
            crate::src::qcommon::q_shared::qfalse
    };
}
