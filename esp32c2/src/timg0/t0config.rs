#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `DIVCNT_RST` writer - When set, Timer 12 's clock divider counter will be reset."]
pub type DIVCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `DIVIDER` reader - Timer 13 clock (T13_clk) prescaler value."]
pub type DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVIDER` writer - Timer 13 clock (T13_clk) prescaler value."]
pub type DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0CONFIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `AUTORELOAD` reader - When set, timer 29 auto-reload at alarm is enabled."]
pub type AUTORELOAD_R = crate::BitReader<bool>;
#[doc = "Field `AUTORELOAD` writer - When set, timer 29 auto-reload at alarm is enabled."]
pub type AUTORELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `INCREASE` reader - When set, the timer 30 time-base counter will increment every clock tick. When cleared, the timer 30 time-base counter will decrement."]
pub type INCREASE_R = crate::BitReader<bool>;
#[doc = "Field `INCREASE` writer - When set, the timer 30 time-base counter will increment every clock tick. When cleared, the timer 30 time-base counter will decrement."]
pub type INCREASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `EN` reader - When set, the timer 31 time-base counter is enabled."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - When set, the timer 31 time-base counter is enabled."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn use_xtal(&self) -> USE_XTAL_R {
        USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer 13 clock (T13_clk) prescaler value."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set, timer 29 auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set, the timer 30 time-base counter will increment every clock tick. When cleared, the timer 30 time-base counter will decrement."]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, the timer 31 time-base counter is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn use_xtal(&mut self) -> USE_XTAL_W<9> {
        USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<10> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bit 12 - When set, Timer 12 's clock divider counter will be reset."]
    #[inline(always)]
    pub fn divcnt_rst(&mut self) -> DIVCNT_RST_W<12> {
        DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer 13 clock (T13_clk) prescaler value."]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W<13> {
        DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set, timer 29 auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<29> {
        AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set, the timer 30 time-base counter will increment every clock tick. When cleared, the timer 30 time-base counter will decrement."]
    #[inline(always)]
    pub fn increase(&mut self) -> INCREASE_W<30> {
        INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set, the timer 31 time-base counter is enabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer $x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2000
    }
}