#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub struct R(crate::R<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub struct W(crate::W<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP_CONF_SPEC>;
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
impl From<crate::W<EXT_WAKEUP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_WAKEUP0_LV` reader - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP0_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP0_LV` writer - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP0_LV_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP_CONF_SPEC, O>;
#[doc = "Field `EXT_WAKEUP1_LV` reader - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP1_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP1_LV` writer - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP1_LV_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&self) -> EXT_WAKEUP0_LV_R {
        EXT_WAKEUP0_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&self) -> EXT_WAKEUP1_LV_R {
        EXT_WAKEUP1_LV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CONF")
            .field(
                "ext_wakeup0_lv",
                &format_args!("{}", self.ext_wakeup0_lv().bit()),
            )
            .field(
                "ext_wakeup1_lv",
                &format_args!("{}", self.ext_wakeup1_lv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup0_lv(&mut self) -> EXT_WAKEUP0_LV_W<30> {
        EXT_WAKEUP0_LV_W::new(self)
    }
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_lv(&mut self) -> EXT_WAKEUP1_LV_W<31> {
        EXT_WAKEUP1_LV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup_conf](index.html) module"]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup_conf::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_conf::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
