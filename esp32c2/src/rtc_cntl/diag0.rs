#[doc = "Register `DIAG0` reader"]
pub struct R(crate::R<DIAG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIAG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIAG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIAG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIAG0` writer"]
pub struct W(crate::W<DIAG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIAG0_SPEC>;
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
impl From<crate::W<DIAG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIAG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW_POWER_DIAG1` reader - Need add desc"]
pub type LOW_POWER_DIAG1_R = crate::FieldReader<u32>;
#[doc = "Field `LOW_POWER_DIAG1` writer - Need add desc"]
pub type LOW_POWER_DIAG1_W<'a, const O: u8> = crate::FieldWriter<'a, DIAG0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIAG0")
            .field(
                "low_power_diag1",
                &format_args!("{}", self.low_power_diag1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIAG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn low_power_diag1(&mut self) -> LOW_POWER_DIAG1_W<0> {
        LOW_POWER_DIAG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diag0](index.html) module"]
pub struct DIAG0_SPEC;
impl crate::RegisterSpec for DIAG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diag0::R](R) reader structure"]
impl crate::Readable for DIAG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diag0::W](W) writer structure"]
impl crate::Writable for DIAG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIAG0 to value 0"]
impl crate::Resettable for DIAG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
