#[doc = "Register `J0_MEM[%s]` reader"]
pub struct R(crate::R<J0_MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<J0_MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<J0_MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<J0_MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `J0_MEM[%s]` writer"]
pub struct W(crate::W<J0_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<J0_MEM_SPEC>;
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
impl From<crate::W<J0_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<J0_MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<J0_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The memory that stores J0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [j0_mem](index.html) module"]
pub struct J0_MEM_SPEC;
impl crate::RegisterSpec for J0_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [j0_mem::R](R) reader structure"]
impl crate::Readable for J0_MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [j0_mem::W](W) writer structure"]
impl crate::Writable for J0_MEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets J0_MEM[%s] to value 0"]
impl crate::Resettable for J0_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
