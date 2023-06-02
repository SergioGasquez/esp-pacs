#[doc = "Register `CORE_1_RCD_PDEBUGLS0ADDR` reader"]
pub struct R(crate::R<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_PDEBUGLS0ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_RCD_PDEBUGLS0ADDR` reader - Core1_pdebug_s0addr"]
pub type CORE_1_RCD_PDEBUGLS0ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core1_pdebug_s0addr"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugls0addr(&self) -> CORE_1_RCD_PDEBUGLS0ADDR_R {
        CORE_1_RCD_PDEBUGLS0ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGLS0ADDR")
            .field(
                "core_1_rcd_pdebugls0addr",
                &format_args!("{}", self.core_1_rcd_pdebugls0addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_RCD_PDEBUGLS0ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_pdebugls0addr](index.html) module"]
pub struct CORE_1_RCD_PDEBUGLS0ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_pdebugls0addr::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGLS0ADDR to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGLS0ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
