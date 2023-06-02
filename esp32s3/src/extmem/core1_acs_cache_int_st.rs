#[doc = "Register `CORE1_ACS_CACHE_INT_ST` reader"]
pub struct R(crate::R<CORE1_ACS_CACHE_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE1_ACS_CACHE_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE1_ACS_CACHE_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE1_ACS_CACHE_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE1_IBUS_ACS_MSK_ICACHE_ST` reader - The bit is used to indicate interrupt by cpu access icache while the core1_ibus is disabled or icache is disabled which include speculative access."]
pub type CORE1_IBUS_ACS_MSK_ICACHE_ST_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_WR_ICACHE_ST` reader - The bit is used to indicate interrupt by ibus trying to write icache"]
pub type CORE1_IBUS_WR_ICACHE_ST_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_REJECT_ST` reader - The bit is used to indicate interrupt by authentication fail."]
pub type CORE1_IBUS_REJECT_ST_R = crate::BitReader;
#[doc = "Field `CORE1_DBUS_ACS_MSK_DCACHE_ST` reader - The bit is used to indicate interrupt by cpu access dcache while the core1_dbus is disabled or dcache is disabled which include speculative access."]
pub type CORE1_DBUS_ACS_MSK_DCACHE_ST_R = crate::BitReader;
#[doc = "Field `CORE1_DBUS_REJECT_ST` reader - The bit is used to indicate interrupt by authentication fail."]
pub type CORE1_DBUS_REJECT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate interrupt by cpu access icache while the core1_ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn core1_ibus_acs_msk_icache_st(&self) -> CORE1_IBUS_ACS_MSK_ICACHE_ST_R {
        CORE1_IBUS_ACS_MSK_ICACHE_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate interrupt by ibus trying to write icache"]
    #[inline(always)]
    pub fn core1_ibus_wr_icache_st(&self) -> CORE1_IBUS_WR_ICACHE_ST_R {
        CORE1_IBUS_WR_ICACHE_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate interrupt by authentication fail."]
    #[inline(always)]
    pub fn core1_ibus_reject_st(&self) -> CORE1_IBUS_REJECT_ST_R {
        CORE1_IBUS_REJECT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate interrupt by cpu access dcache while the core1_dbus is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    pub fn core1_dbus_acs_msk_dcache_st(&self) -> CORE1_DBUS_ACS_MSK_DCACHE_ST_R {
        CORE1_DBUS_ACS_MSK_DCACHE_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate interrupt by authentication fail."]
    #[inline(always)]
    pub fn core1_dbus_reject_st(&self) -> CORE1_DBUS_REJECT_ST_R {
        CORE1_DBUS_REJECT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE1_ACS_CACHE_INT_ST")
            .field(
                "core1_ibus_acs_msk_icache_st",
                &format_args!("{}", self.core1_ibus_acs_msk_icache_st().bit()),
            )
            .field(
                "core1_ibus_wr_icache_st",
                &format_args!("{}", self.core1_ibus_wr_icache_st().bit()),
            )
            .field(
                "core1_ibus_reject_st",
                &format_args!("{}", self.core1_ibus_reject_st().bit()),
            )
            .field(
                "core1_dbus_acs_msk_dcache_st",
                &format_args!("{}", self.core1_dbus_acs_msk_dcache_st().bit()),
            )
            .field(
                "core1_dbus_reject_st",
                &format_args!("{}", self.core1_dbus_reject_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE1_ACS_CACHE_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core1_acs_cache_int_st](index.html) module"]
pub struct CORE1_ACS_CACHE_INT_ST_SPEC;
impl crate::RegisterSpec for CORE1_ACS_CACHE_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core1_acs_cache_int_st::R](R) reader structure"]
impl crate::Readable for CORE1_ACS_CACHE_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE1_ACS_CACHE_INT_ST to value 0"]
impl crate::Resettable for CORE1_ACS_CACHE_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
