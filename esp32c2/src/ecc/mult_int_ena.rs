#[doc = "Register `MULT_INT_ENA` reader"]
pub struct R(crate::R<MULT_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULT_INT_ENA` writer"]
pub struct W(crate::W<MULT_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULT_INT_ENA_SPEC>;
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
impl From<crate::W<MULT_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULT_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALC_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type CALC_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CALC_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type CALC_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MULT_INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done_int_ena(&self) -> CALC_DONE_INT_ENA_R {
        CALC_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done_int_ena(&mut self) -> CALC_DONE_INT_ENA_W<0> {
        CALC_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_int_ena](index.html) module"]
pub struct MULT_INT_ENA_SPEC;
impl crate::RegisterSpec for MULT_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_int_ena::R](R) reader structure"]
impl crate::Readable for MULT_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mult_int_ena::W](W) writer structure"]
impl crate::Writable for MULT_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MULT_INT_ENA to value 0"]
impl crate::Resettable for MULT_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}