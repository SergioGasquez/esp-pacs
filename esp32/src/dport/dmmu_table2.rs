#[doc = "Register `DMMU_TABLE2` reader"]
pub struct R(crate::R<DMMU_TABLE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMMU_TABLE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMMU_TABLE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMMU_TABLE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMMU_TABLE2` writer"]
pub struct W(crate::W<DMMU_TABLE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMMU_TABLE2_SPEC>;
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
impl From<crate::W<DMMU_TABLE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMMU_TABLE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMMU_TABLE2` reader - "]
pub type DMMU_TABLE2_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE2` writer - "]
pub type DMMU_TABLE2_W<'a, const O: u8> = crate::FieldWriter<'a, DMMU_TABLE2_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table2(&self) -> DMMU_TABLE2_R {
        DMMU_TABLE2_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE2")
            .field(
                "dmmu_table2",
                &format_args!("{}", self.dmmu_table2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table2(&mut self) -> DMMU_TABLE2_W<0> {
        DMMU_TABLE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmmu_table2](index.html) module"]
pub struct DMMU_TABLE2_SPEC;
impl crate::RegisterSpec for DMMU_TABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmmu_table2::R](R) reader structure"]
impl crate::Readable for DMMU_TABLE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmmu_table2::W](W) writer structure"]
impl crate::Writable for DMMU_TABLE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_TABLE2 to value 0x02"]
impl crate::Resettable for DMMU_TABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
