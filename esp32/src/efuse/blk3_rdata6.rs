#[doc = "Register `BLK3_RDATA6` reader"]
pub struct R(crate::R<BLK3_RDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_RDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_RDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_RDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK3_DOUT6` reader - read for BLOCK3"]
pub type BLK3_DOUT6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn blk3_dout6(&self) -> BLK3_DOUT6_R {
        BLK3_DOUT6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA6")
            .field("blk3_dout6", &format_args!("{}", self.blk3_dout6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_RDATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_rdata6](index.html) module"]
pub struct BLK3_RDATA6_SPEC;
impl crate::RegisterSpec for BLK3_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_rdata6::R](R) reader structure"]
impl crate::Readable for BLK3_RDATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK3_RDATA6 to value 0"]
impl crate::Resettable for BLK3_RDATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
