#[doc = "Register `IN1` reader"]
pub struct R(crate::R<IN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN1` writer"]
pub struct W(crate::W<IN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN1_SPEC>;
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
impl From<crate::W<IN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NEXT` reader - GPIO input register for GPIO32-53"]
pub type DATA_NEXT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA_NEXT` writer - GPIO input register for GPIO32-53"]
pub type DATA_NEXT_W<'a, const O: u8> = crate::FieldWriter<'a, IN1_SPEC, 22, O, u32, u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO input register for GPIO32-53"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN1")
            .field("data_next", &format_args!("{}", self.data_next().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO input register for GPIO32-53"]
    #[inline(always)]
    #[must_use]
    pub fn data_next(&mut self) -> DATA_NEXT_W<0> {
        DATA_NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO input register for GPIO32-53\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in1](index.html) module"]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in1::R](R) reader structure"]
impl crate::Readable for IN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in1::W](W) writer structure"]
impl crate::Writable for IN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
