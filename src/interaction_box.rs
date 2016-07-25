use raw;
use Vector;

pub struct InteractionBox {
    raw: *mut raw::InteractionBox
}

impl InteractionBox {
    pub unsafe fn from_raw(raw: *mut raw::InteractionBox) -> InteractionBox {
        InteractionBox {
            raw: raw
        }
    }

    pub fn normalize_point(&self, point: &Vector) -> Vector {
        unsafe {
            Vector::from_raw(raw::lm_interaction_box_normalize_point(self.raw, point.as_raw(), true))
        }
    }

    pub fn normalize_point_clamp(&self, point: &Vector, clamp: bool) -> Vector {
        unsafe {
            Vector::from_raw(raw::lm_interaction_box_normalize_point(self.raw, point.as_raw(), clamp))
        }
    }
}

impl Drop for InteractionBox {
    fn drop(&mut self) {
        unsafe {
            raw::lm_interaction_box_delete(self.raw);
        }
    }
}