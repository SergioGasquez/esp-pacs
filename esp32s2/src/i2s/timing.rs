#[doc = "Register `TIMING` reader"]
pub struct R(crate::R<TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING` writer"]
pub struct W(crate::W<TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_SPEC>;
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
impl From<crate::W<TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BCK_IN_DELAY` reader - Number of delay cycles for BCK signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type TX_BCK_IN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_BCK_IN_DELAY` writer - Number of delay cycles for BCK signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type TX_BCK_IN_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_WS_IN_DELAY` reader - Number of delay cycles for WS signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type TX_WS_IN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_WS_IN_DELAY` writer - Number of delay cycles for WS signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type TX_WS_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_BCK_IN_DELAY` reader - Number of delay cycles for BCK signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_BCK_IN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BCK_IN_DELAY` writer - Number of delay cycles for BCK signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_BCK_IN_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_WS_IN_DELAY` reader - Number of delay cycles for WS signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_WS_IN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_WS_IN_DELAY` writer - Number of delay cycles for WS signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_WS_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_SD_IN_DELAY` reader - Number of delay cycles for SD signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_SD_IN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_SD_IN_DELAY` writer - Number of delay cycles for SD signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type RX_SD_IN_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_BCK_OUT_DELAY` reader - Number of delay cycles for BCK signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_BCK_OUT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_BCK_OUT_DELAY` writer - Number of delay cycles for BCK signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_BCK_OUT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_WS_OUT_DELAY` reader - Number of delay cycles for WS signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_WS_OUT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_WS_OUT_DELAY` writer - Number of delay cycles for WS signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_WS_OUT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_SD_OUT_DELAY` reader - Number of delay cycles for SD signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_SD_OUT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SD_OUT_DELAY` writer - Number of delay cycles for SD signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type TX_SD_OUT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_WS_OUT_DELAY` reader - Number of delay cycles for WS signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type RX_WS_OUT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_WS_OUT_DELAY` writer - Number of delay cycles for WS signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type RX_WS_OUT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_BCK_OUT_DELAY` reader - Number of delay cycles for BCK signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type RX_BCK_OUT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BCK_OUT_DELAY` writer - Number of delay cycles for BCK signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
pub type RX_BCK_OUT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_DSYNC_SW` reader - Set this bit to synchronize signals into the transmitter by two flip-flop synchronizer. 0: the signals will be firstly clocked by rising clock edge , then clocked by falling clock edge. 1: the signals will be firstly clocked by falling clock edge, then clocked by rising clock edge."]
pub type TX_DSYNC_SW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DSYNC_SW` writer - Set this bit to synchronize signals into the transmitter by two flip-flop synchronizer. 0: the signals will be firstly clocked by rising clock edge , then clocked by falling clock edge. 1: the signals will be firstly clocked by falling clock edge, then clocked by rising clock edge."]
pub type TX_DSYNC_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMING_SPEC, bool, O>;
#[doc = "Field `RX_DSYNC_SW` reader - Set this bit to synchronize signals into the receiver by two flip-flop synchronizer. 0: the signals will be clocked by rising clock edge firstly, then clocked by falling clock edge. 1: the signals will be clocked by falling clock edge firstly, then clocked by rising clock edge."]
pub type RX_DSYNC_SW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DSYNC_SW` writer - Set this bit to synchronize signals into the receiver by two flip-flop synchronizer. 0: the signals will be clocked by rising clock edge firstly, then clocked by falling clock edge. 1: the signals will be clocked by falling clock edge firstly, then clocked by rising clock edge."]
pub type RX_DSYNC_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMING_SPEC, bool, O>;
#[doc = "Field `DATA_ENABLE_DELAY` reader - Number of delay cycles for data valid flag based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type DATA_ENABLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_ENABLE_DELAY` writer - Number of delay cycles for data valid flag based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
pub type DATA_ENABLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_BCK_IN_INV` reader - Set this bit to invert BCK signal input to the slave transmitter."]
pub type TX_BCK_IN_INV_R = crate::BitReader<bool>;
#[doc = "Field `TX_BCK_IN_INV` writer - Set this bit to invert BCK signal input to the slave transmitter."]
pub type TX_BCK_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMING_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Number of delay cycles for BCK signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn tx_bck_in_delay(&self) -> TX_BCK_IN_DELAY_R {
        TX_BCK_IN_DELAY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of delay cycles for WS signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn tx_ws_in_delay(&self) -> TX_WS_IN_DELAY_R {
        TX_WS_IN_DELAY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Number of delay cycles for BCK signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_bck_in_delay(&self) -> RX_BCK_IN_DELAY_R {
        RX_BCK_IN_DELAY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Number of delay cycles for WS signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_ws_in_delay(&self) -> RX_WS_IN_DELAY_R {
        RX_WS_IN_DELAY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Number of delay cycles for SD signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_sd_in_delay(&self) -> RX_SD_IN_DELAY_R {
        RX_SD_IN_DELAY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Number of delay cycles for BCK signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_bck_out_delay(&self) -> TX_BCK_OUT_DELAY_R {
        TX_BCK_OUT_DELAY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Number of delay cycles for WS signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_ws_out_delay(&self) -> TX_WS_OUT_DELAY_R {
        TX_WS_OUT_DELAY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Number of delay cycles for SD signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_sd_out_delay(&self) -> TX_SD_OUT_DELAY_R {
        TX_SD_OUT_DELAY_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Number of delay cycles for WS signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn rx_ws_out_delay(&self) -> RX_WS_OUT_DELAY_R {
        RX_WS_OUT_DELAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Number of delay cycles for BCK signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn rx_bck_out_delay(&self) -> RX_BCK_OUT_DELAY_R {
        RX_BCK_OUT_DELAY_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Set this bit to synchronize signals into the transmitter by two flip-flop synchronizer. 0: the signals will be firstly clocked by rising clock edge , then clocked by falling clock edge. 1: the signals will be firstly clocked by falling clock edge, then clocked by rising clock edge."]
    #[inline(always)]
    pub fn tx_dsync_sw(&self) -> TX_DSYNC_SW_R {
        TX_DSYNC_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to synchronize signals into the receiver by two flip-flop synchronizer. 0: the signals will be clocked by rising clock edge firstly, then clocked by falling clock edge. 1: the signals will be clocked by falling clock edge firstly, then clocked by rising clock edge."]
    #[inline(always)]
    pub fn rx_dsync_sw(&self) -> RX_DSYNC_SW_R {
        RX_DSYNC_SW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Number of delay cycles for data valid flag based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn data_enable_delay(&self) -> DATA_ENABLE_DELAY_R {
        DATA_ENABLE_DELAY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Set this bit to invert BCK signal input to the slave transmitter."]
    #[inline(always)]
    pub fn tx_bck_in_inv(&self) -> TX_BCK_IN_INV_R {
        TX_BCK_IN_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of delay cycles for BCK signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn tx_bck_in_delay(&mut self) -> TX_BCK_IN_DELAY_W<0> {
        TX_BCK_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 2:3 - Number of delay cycles for WS signal into the transmitter based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn tx_ws_in_delay(&mut self) -> TX_WS_IN_DELAY_W<2> {
        TX_WS_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 4:5 - Number of delay cycles for BCK signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_bck_in_delay(&mut self) -> RX_BCK_IN_DELAY_W<4> {
        RX_BCK_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 6:7 - Number of delay cycles for WS signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_ws_in_delay(&mut self) -> RX_WS_IN_DELAY_W<6> {
        RX_WS_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 8:9 - Number of delay cycles for SD signal into the receiver based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn rx_sd_in_delay(&mut self) -> RX_SD_IN_DELAY_W<8> {
        RX_SD_IN_DELAY_W::new(self)
    }
    #[doc = "Bits 10:11 - Number of delay cycles for BCK signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_bck_out_delay(&mut self) -> TX_BCK_OUT_DELAY_W<10> {
        TX_BCK_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 12:13 - Number of delay cycles for WS signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_ws_out_delay(&mut self) -> TX_WS_OUT_DELAY_W<12> {
        TX_WS_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 14:15 - Number of delay cycles for SD signal out of the transmitter based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn tx_sd_out_delay(&mut self) -> TX_SD_OUT_DELAY_W<14> {
        TX_SD_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 16:17 - Number of delay cycles for WS signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn rx_ws_out_delay(&mut self) -> RX_WS_OUT_DELAY_W<16> {
        RX_WS_OUT_DELAY_W::new(self)
    }
    #[doc = "Bits 18:19 - Number of delay cycles for BCK signal out of the receiver based on I2S0_CLK. 0: delayed by 0 cycle. 1: delayed by 1 cycle. 2: delayed by 2 cycles. 3: delayed by 3 cycles."]
    #[inline(always)]
    pub fn rx_bck_out_delay(&mut self) -> RX_BCK_OUT_DELAY_W<18> {
        RX_BCK_OUT_DELAY_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to synchronize signals into the transmitter by two flip-flop synchronizer. 0: the signals will be firstly clocked by rising clock edge , then clocked by falling clock edge. 1: the signals will be firstly clocked by falling clock edge, then clocked by rising clock edge."]
    #[inline(always)]
    pub fn tx_dsync_sw(&mut self) -> TX_DSYNC_SW_W<20> {
        TX_DSYNC_SW_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to synchronize signals into the receiver by two flip-flop synchronizer. 0: the signals will be clocked by rising clock edge firstly, then clocked by falling clock edge. 1: the signals will be clocked by falling clock edge firstly, then clocked by rising clock edge."]
    #[inline(always)]
    pub fn rx_dsync_sw(&mut self) -> RX_DSYNC_SW_W<21> {
        RX_DSYNC_SW_W::new(self)
    }
    #[doc = "Bits 22:23 - Number of delay cycles for data valid flag based on I2S0_CLK. 0: delayed by 1.5 cycles. 1: delayed by 2.5 cycles. 2: delayed by 3.5 cycles. 3: delayed by 4.5 cycles."]
    #[inline(always)]
    pub fn data_enable_delay(&mut self) -> DATA_ENABLE_DELAY_W<22> {
        DATA_ENABLE_DELAY_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to invert BCK signal input to the slave transmitter."]
    #[inline(always)]
    pub fn tx_bck_in_inv(&mut self) -> TX_BCK_IN_INV_W<24> {
        TX_BCK_IN_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](index.html) module"]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing::R](R) reader structure"]
impl crate::Readable for TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing::W](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}