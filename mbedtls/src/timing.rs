use mbedtls_sys::*;
use crate::error::{Result};
use std::mem::MaybeUninit;

define!(
    #[c_ty(timing_delay_context)]
    struct TimingDelayContext;
    impl<'a> Into<ptr> {}
    impl<'a> UnsafeFrom<ptr> {}
);

impl TimingDelayContext {
    pub fn new() -> Self {
        let mut inner: MaybeUninit<TimingDelayContext> = MaybeUninit::uninit();
        unsafe {
            let ctx = inner.assume_init();

            return ctx;
        }
    }
}

