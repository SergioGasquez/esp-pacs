#[doc = "Register `_0_TO_EOF_BFR_DES_ADDR` reader"]
pub struct R(crate::R<_0_TO_EOF_BFR_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_TO_EOF_BFR_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_TO_EOF_BFR_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_TO_EOF_BFR_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_TO_EOF_BFR_DES_ADDR` reader - "]
pub type SLC0_TO_EOF_BFR_DES_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_to_eof_bfr_des_addr(&self) -> SLC0_TO_EOF_BFR_DES_ADDR_R {
        SLC0_TO_EOF_BFR_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TO_EOF_BFR_DES_ADDR")
            .field(
                "slc0_to_eof_bfr_des_addr",
                &format_args!("{}", self.slc0_to_eof_bfr_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_TO_EOF_BFR_DES_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_to_eof_bfr_des_addr](index.html) module"]
pub struct _0_TO_EOF_BFR_DES_ADDR_SPEC;
impl crate::RegisterSpec for _0_TO_EOF_BFR_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_to_eof_bfr_des_addr::R](R) reader structure"]
impl crate::Readable for _0_TO_EOF_BFR_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_TO_EOF_BFR_DES_ADDR to value 0"]
impl crate::Resettable for _0_TO_EOF_BFR_DES_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
