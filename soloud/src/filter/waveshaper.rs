use crate::prelude::*;
use super::ParamType;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum WaveShaperFilterAttr {
    Wet = 0,
    Amount = 1,
}

#[derive(FilterExt)]
pub struct WaveShaperFilter {
    _inner: *mut soloud_sys::soloud::WaveShaperFilter,
}

impl WaveShaperFilter {
    pub fn set_params(&mut self, aAmount: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::WaveShaperFilter_setParams(self._inner, aAmount);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
