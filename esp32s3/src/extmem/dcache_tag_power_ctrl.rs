#[doc = "Register `DCACHE_TAG_POWER_CTRL` reader"]
pub struct R(crate::R<DCACHE_TAG_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_TAG_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_TAG_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_TAG_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_TAG_POWER_CTRL` writer"]
pub struct W(crate::W<DCACHE_TAG_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_TAG_POWER_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_TAG_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_TAG_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
pub type DCACHE_TAG_MEM_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
pub type DCACHE_TAG_MEM_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_TAG_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
pub type DCACHE_TAG_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PD` writer - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
pub type DCACHE_TAG_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_TAG_POWER_CTRL_SPEC, bool, O>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
pub type DCACHE_TAG_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PU` writer - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
pub type DCACHE_TAG_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCACHE_TAG_POWER_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn dcache_tag_mem_force_on(&self) -> DCACHE_TAG_MEM_FORCE_ON_R {
        DCACHE_TAG_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pd(&self) -> DCACHE_TAG_MEM_FORCE_PD_R {
        DCACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pu(&self) -> DCACHE_TAG_MEM_FORCE_PU_R {
        DCACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn dcache_tag_mem_force_on(&mut self) -> DCACHE_TAG_MEM_FORCE_ON_W<0> {
        DCACHE_TAG_MEM_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pd(&mut self) -> DCACHE_TAG_MEM_FORCE_PD_W<1> {
        DCACHE_TAG_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pu(&mut self) -> DCACHE_TAG_MEM_FORCE_PU_W<2> {
        DCACHE_TAG_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_tag_power_ctrl](index.html) module"]
pub struct DCACHE_TAG_POWER_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_TAG_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_tag_power_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_TAG_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_tag_power_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_TAG_POWER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCACHE_TAG_POWER_CTRL to value 0x05"]
impl crate::Resettable for DCACHE_TAG_POWER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}