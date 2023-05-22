#[doc = "Register `LP_INT_RAW` reader"]
pub struct R(crate::R<LP_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_INT_RAW` writer"]
pub struct W(crate::W<LP_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_MODE0_LP_INT_RAW` reader - need_des"]
pub type BOD_MODE0_LP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `BOD_MODE0_LP_INT_RAW` writer - need_des"]
pub type BOD_MODE0_LP_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0_lp_int_raw(&self) -> BOD_MODE0_LP_INT_RAW_R {
        BOD_MODE0_LP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0_lp_int_raw(&mut self) -> BOD_MODE0_LP_INT_RAW_W<31> {
        BOD_MODE0_LP_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_raw](index.html) module"]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_int_raw::R](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_int_raw::W](W) writer structure"]
impl crate::Writable for LP_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}