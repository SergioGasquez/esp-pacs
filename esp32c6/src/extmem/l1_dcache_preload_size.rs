#[doc = "Register `L1_DCACHE_PRELOAD_SIZE` reader"]
pub struct R(crate::R<L1_DCACHE_PRELOAD_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_DCACHE_PRELOAD_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_DCACHE_PRELOAD_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_DCACHE_PRELOAD_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_DCACHE_PRELOAD_SIZE` writer"]
pub struct W(crate::W<L1_DCACHE_PRELOAD_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_DCACHE_PRELOAD_SIZE_SPEC>;
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
impl From<crate::W<L1_DCACHE_PRELOAD_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_DCACHE_PRELOAD_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
pub type L1_CACHE_PRELOAD_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L1_CACHE_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
pub type L1_CACHE_PRELOAD_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1_DCACHE_PRELOAD_SIZE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_cache_preload_size(&self) -> L1_CACHE_PRELOAD_SIZE_R {
        L1_CACHE_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_preload_size(&mut self) -> L1_CACHE_PRELOAD_SIZE_W<0> {
        L1_CACHE_PRELOAD_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 Cache preload size configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_dcache_preload_size](index.html) module"]
pub struct L1_DCACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for L1_DCACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_dcache_preload_size::R](R) reader structure"]
impl crate::Readable for L1_DCACHE_PRELOAD_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_dcache_preload_size::W](W) writer structure"]
impl crate::Writable for L1_DCACHE_PRELOAD_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1_DCACHE_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}