#[doc = "Register `START` writer"]
pub struct W(crate::W<START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_SPEC>;
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
impl From<crate::W<START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_START` writer - Set this bit to start encryption operation on data buffer."]
pub type FLASH_START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, START_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Set this bit to start encryption operation on data buffer."]
    #[inline(always)]
    pub fn flash_start(&mut self) -> FLASH_START_W<0> {
        FLASH_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](index.html) module"]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [start::W](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}