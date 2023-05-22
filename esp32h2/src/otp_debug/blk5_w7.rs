#[doc = "Register `BLK5_W7` reader"]
pub struct R(crate::R<BLK5_W7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK5_W7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK5_W7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK5_W7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK5_W7` reader - Otp block5 word7 data."]
pub type BLOCK5_W7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word7 data."]
    #[inline(always)]
    pub fn block5_w7(&self) -> BLOCK5_W7_R {
        BLOCK5_W7_R::new(self.bits)
    }
}
#[doc = "Otp debuger block5 data register7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk5_w7](index.html) module"]
pub struct BLK5_W7_SPEC;
impl crate::RegisterSpec for BLK5_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk5_w7::R](R) reader structure"]
impl crate::Readable for BLK5_W7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK5_W7 to value 0"]
impl crate::Resettable for BLK5_W7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}