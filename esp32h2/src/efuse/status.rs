#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE` reader - Indicates the state of the eFuse state machine."]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTP_LOAD_SW` reader - The value of OTP_LOAD_SW."]
pub type OTP_LOAD_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTP_VDDQ_C_SYNC2` reader - The value of OTP_VDDQ_C_SYNC2."]
pub type OTP_VDDQ_C_SYNC2_R = crate::BitReader<bool>;
#[doc = "Field `OTP_STROBE_SW` reader - The value of OTP_STROBE_SW."]
pub type OTP_STROBE_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTP_CSB_SW` reader - The value of OTP_CSB_SW."]
pub type OTP_CSB_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTP_PGENB_SW` reader - The value of OTP_PGENB_SW."]
pub type OTP_PGENB_SW_R = crate::BitReader<bool>;
#[doc = "Field `OTP_VDDQ_IS_SW` reader - The value of OTP_VDDQ_IS_SW."]
pub type OTP_VDDQ_IS_SW_R = crate::BitReader<bool>;
#[doc = "Field `BLK0_VALID_BIT_CNT` reader - Indicates the number of block valid bit."]
pub type BLK0_VALID_BIT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CUR_ECDSA_BLK` reader - Indicates which block is used for ECDSA key output."]
pub type CUR_ECDSA_BLK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Indicates the state of the eFuse state machine."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The value of OTP_LOAD_SW."]
    #[inline(always)]
    pub fn otp_load_sw(&self) -> OTP_LOAD_SW_R {
        OTP_LOAD_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The value of OTP_VDDQ_C_SYNC2."]
    #[inline(always)]
    pub fn otp_vddq_c_sync2(&self) -> OTP_VDDQ_C_SYNC2_R {
        OTP_VDDQ_C_SYNC2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The value of OTP_STROBE_SW."]
    #[inline(always)]
    pub fn otp_strobe_sw(&self) -> OTP_STROBE_SW_R {
        OTP_STROBE_SW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of OTP_CSB_SW."]
    #[inline(always)]
    pub fn otp_csb_sw(&self) -> OTP_CSB_SW_R {
        OTP_CSB_SW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of OTP_PGENB_SW."]
    #[inline(always)]
    pub fn otp_pgenb_sw(&self) -> OTP_PGENB_SW_R {
        OTP_PGENB_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of OTP_VDDQ_IS_SW."]
    #[inline(always)]
    pub fn otp_vddq_is_sw(&self) -> OTP_VDDQ_IS_SW_R {
        OTP_VDDQ_IS_SW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:19 - Indicates the number of block valid bit."]
    #[inline(always)]
    pub fn blk0_valid_bit_cnt(&self) -> BLK0_VALID_BIT_CNT_R {
        BLK0_VALID_BIT_CNT_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23 - Indicates which block is used for ECDSA key output."]
    #[inline(always)]
    pub fn cur_ecdsa_blk(&self) -> CUR_ECDSA_BLK_R {
        CUR_ECDSA_BLK_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "eFuse status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
