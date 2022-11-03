#[doc = "Register `BLK9_W9` reader"]
pub struct R(crate::R<BLK9_W9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK9_W9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK9_W9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK9_W9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK9_W9` reader - Otp block9 word9 data."]
pub type BLOCK9_W9_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word9 data."]
    #[inline(always)]
    pub fn block9_w9(&self) -> BLOCK9_W9_R {
        BLOCK9_W9_R::new(self.bits)
    }
}
#[doc = "Otp debuger block9 data register9.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk9_w9](index.html) module"]
pub struct BLK9_W9_SPEC;
impl crate::RegisterSpec for BLK9_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk9_w9::R](R) reader structure"]
impl crate::Readable for BLK9_W9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK9_W9 to value 0"]
impl crate::Resettable for BLK9_W9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}