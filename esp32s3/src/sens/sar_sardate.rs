#[doc = "Register `SAR_SARDATE` reader"]
pub struct R(crate::R<SAR_SARDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SARDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SARDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SARDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_SARDATE` writer"]
pub struct W(crate::W<SAR_SARDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SARDATE_SPEC>;
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
impl From<crate::W<SAR_SARDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SARDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DATE` reader - version"]
pub type SAR_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SAR_DATE` writer - version"]
pub type SAR_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_SARDATE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - version"]
    #[inline(always)]
    pub fn sar_date(&self) -> SAR_DATE_R {
        SAR_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - version"]
    #[inline(always)]
    pub fn sar_date(&mut self) -> SAR_DATE_W<0> {
        SAR_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_sardate](index.html) module"]
pub struct SAR_SARDATE_SPEC;
impl crate::RegisterSpec for SAR_SARDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_sardate::R](R) reader structure"]
impl crate::Readable for SAR_SARDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_sardate::W](W) writer structure"]
impl crate::Writable for SAR_SARDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_SARDATE to value 0x0210_1180"]
impl crate::Resettable for SAR_SARDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_1180
    }
}