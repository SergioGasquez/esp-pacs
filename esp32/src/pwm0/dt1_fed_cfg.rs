#[doc = "Register `DT1_FED_CFG` reader"]
pub struct R(crate::R<DT1_FED_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT1_FED_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT1_FED_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT1_FED_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT1_FED_CFG` writer"]
pub struct W(crate::W<DT1_FED_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT1_FED_CFG_SPEC>;
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
impl From<crate::W<DT1_FED_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT1_FED_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT1_FED` reader - "]
pub type DT1_FED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DT1_FED` writer - "]
pub type DT1_FED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT1_FED_CFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt1_fed(&self) -> DT1_FED_R {
        DT1_FED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt1_fed(&mut self) -> DT1_FED_W<0> {
        DT1_FED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1_fed_cfg](index.html) module"]
pub struct DT1_FED_CFG_SPEC;
impl crate::RegisterSpec for DT1_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt1_fed_cfg::R](R) reader structure"]
impl crate::Readable for DT1_FED_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt1_fed_cfg::W](W) writer structure"]
impl crate::Writable for DT1_FED_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT1_FED_CFG to value 0"]
impl crate::Resettable for DT1_FED_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}