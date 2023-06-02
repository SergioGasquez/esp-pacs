#[doc = "Register `BLK5_W11` reader"]
pub struct R(crate::R<BLK5_W11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK5_W11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK5_W11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK5_W11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK5_W11` reader - Otp block5 word11 data."]
pub type BLOCK5_W11_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word11 data."]
    #[inline(always)]
    pub fn block5_w11(&self) -> BLOCK5_W11_R {
        BLOCK5_W11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK5_W11")
            .field("block5_w11", &format_args!("{}", self.block5_w11().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK5_W11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block5 data register11.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk5_w11](index.html) module"]
pub struct BLK5_W11_SPEC;
impl crate::RegisterSpec for BLK5_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk5_w11::R](R) reader structure"]
impl crate::Readable for BLK5_W11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK5_W11 to value 0"]
impl crate::Resettable for BLK5_W11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
