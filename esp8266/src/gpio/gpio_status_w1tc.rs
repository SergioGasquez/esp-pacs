#[doc = "Register `GPIO_STATUS_W1TC` writer"]
pub struct W(crate::W<GPIO_STATUS_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_STATUS_W1TC_SPEC>;
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
impl From<crate::W<GPIO_STATUS_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_STATUS_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_STATUS_INTERRUPT_W1TC` writer - Writing 1 into a bit in this register will clear the related bit in GPIO_STATUS_INTERRUPT"]
pub type GPIO_STATUS_INTERRUPT_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_STATUS_W1TC_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will clear the related bit in GPIO_STATUS_INTERRUPT"]
    #[inline(always)]
    pub fn gpio_status_interrupt_w1tc(&mut self) -> GPIO_STATUS_INTERRUPT_W1TC_W<0> {
        GPIO_STATUS_INTERRUPT_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_STATUS_W1TC\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status_w1tc](index.html) module"]
pub struct GPIO_STATUS_W1TC_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_status_w1tc::W](W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_STATUS_W1TC to value 0"]
impl crate::Resettable for GPIO_STATUS_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}