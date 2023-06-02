#[doc = "Register `BACKUP_BUS_PMS_MONITOR_2` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR` reader - Record BackUp illegal access interrupt state."]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS` reader - Record htrans when BackUp initate illegal access."]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE` reader - Record access type when BackUp initate illegal access."]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE` reader - Record access direction when BackUp initiate illegal access."]
pub type BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Record BackUp illegal access interrupt state."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_intr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Record htrans when BackUp initate illegal access."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_htrans(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Record access type when BackUp initate illegal access."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_hsize(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Record access direction when BackUp initiate illegal access."]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_hwrite(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_MONITOR_2")
            .field(
                "backup_bus_pms_monitor_violate_intr",
                &format_args!("{}", self.backup_bus_pms_monitor_violate_intr().bit()),
            )
            .field(
                "backup_bus_pms_monitor_violate_status_htrans",
                &format_args!(
                    "{}",
                    self.backup_bus_pms_monitor_violate_status_htrans().bits()
                ),
            )
            .field(
                "backup_bus_pms_monitor_violate_status_hsize",
                &format_args!(
                    "{}",
                    self.backup_bus_pms_monitor_violate_status_hsize().bits()
                ),
            )
            .field(
                "backup_bus_pms_monitor_violate_status_hwrite",
                &format_args!(
                    "{}",
                    self.backup_bus_pms_monitor_violate_status_hwrite().bit()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_MONITOR_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BackUp permission report register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_monitor_2](index.html) module"]
pub struct BACKUP_BUS_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
