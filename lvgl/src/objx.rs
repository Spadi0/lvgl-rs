use core::ptr;
use cty;
use lvgl_sys;

pub trait Container {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

pub struct ObjectX {
    raw: ptr::NonNull<lvgl_sys::lv_obj_t>,
}

impl ObjectX {
    pub(crate) fn new(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw }
    }
}

impl Container for ObjectX {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        unsafe { ptr::NonNull::new_unchecked(self.raw.as_ptr()) }
    }
}

pub trait Object: Container {
    fn set_pos(&mut self, x: i16, y: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_pos(
                self.raw().as_mut(),
                x as lvgl_sys::lv_coord_t,
                y as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_size(&mut self, w: i16, h: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_size(
                self.raw().as_mut(),
                w as lvgl_sys::lv_coord_t,
                h as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_width(&mut self, w: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_width(self.raw().as_mut(), w as lvgl_sys::lv_coord_t);
        }
    }

    fn set_height(&mut self, h: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_height(self.raw().as_mut(), h as lvgl_sys::lv_coord_t);
        }
    }

    fn set_object_align<C>(
        &mut self,
        base: &mut C,
        align: ObjectAlign,
        x_mod: i32,
        y_mod: i32,
    ) where C: Container {
        let align = match align {
            ObjectAlign::Center => lvgl_sys::LV_ALIGN_CENTER,
            ObjectAlign::InTopLeft => lvgl_sys::LV_ALIGN_IN_TOP_LEFT,
            ObjectAlign::InTopMid => lvgl_sys::LV_ALIGN_IN_TOP_MID,
            ObjectAlign::InTopRight => lvgl_sys::LV_ALIGN_IN_TOP_RIGHT,
            ObjectAlign::InBottomLeft => lvgl_sys::LV_ALIGN_IN_BOTTOM_LEFT,
            ObjectAlign::InBottomMid => lvgl_sys::LV_ALIGN_IN_BOTTOM_MID,
            ObjectAlign::InBottomRight => lvgl_sys::LV_ALIGN_IN_BOTTOM_RIGHT,
            ObjectAlign::InLeftMid => lvgl_sys::LV_ALIGN_IN_LEFT_MID,
            ObjectAlign::InRightMid => lvgl_sys::LV_ALIGN_IN_RIGHT_MID,
            ObjectAlign::OutTopLeft => lvgl_sys::LV_ALIGN_OUT_TOP_LEFT,
            ObjectAlign::OutTopMid => lvgl_sys::LV_ALIGN_OUT_TOP_MID,
            ObjectAlign::OutTopRight => lvgl_sys::LV_ALIGN_OUT_TOP_RIGHT,
            ObjectAlign::OutBottomLeft => lvgl_sys::LV_ALIGN_OUT_BOTTOM_LEFT,
            ObjectAlign::OutBottomMid => lvgl_sys::LV_ALIGN_OUT_BOTTOM_MID,
            ObjectAlign::OutBottomRight => lvgl_sys::LV_ALIGN_OUT_BOTTOM_RIGHT,
            ObjectAlign::OutLeftTop => lvgl_sys::LV_ALIGN_OUT_LEFT_TOP,
            ObjectAlign::OutLeftMid => lvgl_sys::LV_ALIGN_OUT_LEFT_MID,
            ObjectAlign::OutLeftBottom => lvgl_sys::LV_ALIGN_OUT_LEFT_BOTTOM,
            ObjectAlign::OutRightTop => lvgl_sys::LV_ALIGN_OUT_RIGHT_TOP,
            ObjectAlign::OutRightMid => lvgl_sys::LV_ALIGN_OUT_RIGHT_MID,
            ObjectAlign::OutRightBottom => lvgl_sys::LV_ALIGN_OUT_RIGHT_BOTTOM,
        } as lvgl_sys::lv_align_t;
        unsafe {
            lvgl_sys::lv_obj_align(
                self.raw().as_mut(),
                base.raw().as_mut(),
                align,
                x_mod as lvgl_sys::lv_coord_t,
                y_mod as lvgl_sys::lv_coord_t,
            );
        }
    }
}

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item {
            core: ObjectX,
        }

        impl Container for $item {
            fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl Object for $item {}
    }
}

pub enum ObjectAlign {
    Center,
    InTopLeft,
    InTopMid,
    InTopRight,
    InBottomLeft,
    InBottomMid,
    InBottomRight,
    InLeftMid,
    InRightMid,
    OutTopLeft,
    OutTopMid,
    OutTopRight,
    OutBottomLeft,
    OutBottomMid,
    OutBottomRight,
    OutLeftTop,
    OutLeftMid,
    OutLeftBottom,
    OutRightTop,
    OutRightMid,
    OutRightBottom,
}

define_object!(Button);

impl Button {
    pub fn new<C>(parent: &mut C) -> Self where C: Container {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_btn_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::new(raw);
        Self { core }
    }
}

pub enum LabelAlign {
    Left,
    Center,
    Right,
    Auto,
}

define_object!(Label);

impl Label {
    pub fn new<C>(parent: &mut C) -> Self where C: Container {
        let raw = unsafe {
            let ptr = lvgl_sys::lv_label_create(parent.raw().as_mut(), ptr::null_mut());
            ptr::NonNull::new_unchecked(ptr)
        };
        let core = ObjectX::new(raw);
        Self { core }
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            lvgl_sys::lv_label_set_text(
                self.core.raw().as_mut(),
                text.as_ptr() as *const cty::c_char,
            );
        }
    }

    pub fn set_align(&mut self, align: LabelAlign) {
        let align = match align {
            LabelAlign::Left => lvgl_sys::LV_LABEL_ALIGN_LEFT,
            LabelAlign::Center => lvgl_sys::LV_LABEL_ALIGN_CENTER,
            LabelAlign::Right => lvgl_sys::LV_LABEL_ALIGN_RIGHT,
            LabelAlign::Auto => lvgl_sys::LV_LABEL_ALIGN_AUTO,
        } as lvgl_sys::lv_label_align_t;
        unsafe {
            lvgl_sys::lv_label_set_align(self.core.raw().as_mut(), align);
        }
    }
}