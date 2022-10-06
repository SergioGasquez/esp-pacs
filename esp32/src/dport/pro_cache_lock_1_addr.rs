#[doc = "Register `PRO_CACHE_LOCK_1_ADDR` reader"]
pub struct R(crate::R<PRO_CACHE_LOCK_1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_LOCK_1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_LOCK_1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_LOCK_1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_LOCK_1_ADDR` writer"]
pub struct W(crate::W<PRO_CACHE_LOCK_1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_LOCK_1_ADDR_SPEC>;
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
impl From<crate::W<PRO_CACHE_LOCK_1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_LOCK_1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE` reader - "]
pub type PRE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE` writer - "]
pub type PRE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CACHE_LOCK_1_ADDR_SPEC, u16, u16, 14, O>;
#[doc = "Field `MIN` reader - "]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - "]
pub type MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CACHE_LOCK_1_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAX` reader - "]
pub type MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX` writer - "]
pub type MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CACHE_LOCK_1_ADDR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W<0> {
        PRE_W::new(self)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W<14> {
        MIN_W::new(self)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&mut self) -> MAX_W<18> {
        MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_lock_1_addr](index.html) module"]
pub struct PRO_CACHE_LOCK_1_ADDR_SPEC;
impl crate::RegisterSpec for PRO_CACHE_LOCK_1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_lock_1_addr::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_lock_1_addr::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CACHE_LOCK_1_ADDR to value 0"]
impl crate::Resettable for PRO_CACHE_LOCK_1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}