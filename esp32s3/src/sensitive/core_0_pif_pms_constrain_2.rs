#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` reader - Core0 access bt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` writer - Core0 access bt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` reader - Core0 access i2c_ext0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` writer - Core0 access i2c_ext0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` reader - Core0 access uhci0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` writer - Core0 access uhci0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST` reader - Core0 access slchost permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST` writer - Core0 access slchost permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` reader - Core0 access rmt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` writer - Core0 access rmt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT` reader - Core0 access pcnt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT` writer - Core0 access pcnt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC` reader - Core0 access slc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC` writer - Core0 access slc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` reader - Core0 access ledc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` writer - Core0 access ledc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP` reader - Core0 access backup permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP` writer - Core0 access backup permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` reader - Core0 access bb permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` writer - Core0 access bb permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0` reader - Core0 access pwm0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0` writer - Core0 access pwm0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` reader - Core0 access timergroup permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` writer - Core0 access timergroup permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` reader - Core0 access timergroup1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` writer - Core0 access timergroup1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` reader - Core0 access systimer permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` writer - Core0 access systimer permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access bt permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access i2c_ext0 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access uhci0 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access slchost permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_slchost(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access rmt permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rmt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core0 access pcnt permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pcnt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access slc permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_slc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access ledc permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ledc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Core0 access backup permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_backup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core0 access bb permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core0 access pwm0 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pwm0(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access timergroup permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access timergroup1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core0 access systimer permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_2")
            .field(
                "core_0_pif_pms_constrain_world_0_bt",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_bt().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_i2c_ext0",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_i2c_ext0().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_uhci0",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_uhci0().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_slchost",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_slchost().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_rmt",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_rmt().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_pcnt",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_pcnt().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_slc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_slc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_ledc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_ledc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_backup",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_backup().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_bb",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_bb().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_pwm0",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_pwm0().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_timergroup",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_timergroup().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_timergroup1",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_timergroup1().bits()
                ),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_systimer",
                &format_args!(
                    "{}",
                    self.core_0_pif_pms_constrain_world_0_systimer().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access bt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_bt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access i2c_ext0 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access uhci0 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access slchost permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_slchost(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLCHOST_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access rmt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_rmt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W::new(self)
    }
    #[doc = "Bits 12:13 - Core0 access pcnt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_pcnt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_W<12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PCNT_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access slc permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_slc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SLC_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access ledc permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_ledc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W::new(self)
    }
    #[doc = "Bits 18:19 - Core0 access backup permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_backup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_W<18> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BACKUP_W::new(self)
    }
    #[doc = "Bits 22:23 - Core0 access bb permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_bb(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W<22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W::new(self)
    }
    #[doc = "Bits 24:25 - Core0 access pwm0 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_pwm0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM0_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access timergroup permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access timergroup1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W::new(self)
    }
    #[doc = "Bits 30:31 - Core0 access systimer permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_2](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_2 to value 0xffcf_fff3"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffcf_fff3;
}
