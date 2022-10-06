#[doc = "Register `SPI_ADDR` reader"]
pub struct R(crate::R<SPI_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_ADDR` writer"]
pub struct W(crate::W<SPI_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_ADDR_SPEC>;
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
impl From<crate::W<SPI_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `iodata_start_addr` reader - In the master mode, it is the value of address in \"address\" phase."]
pub type IODATA_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `iodata_start_addr` writer - In the master mode, it is the value of address in \"address\" phase."]
pub type IODATA_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_ADDR_SPEC, u32, u32, 32, O>;
#[doc = "Field `address` reader - "]
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `address` writer - "]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_ADDR_SPEC, u32, u32, 24, O>;
#[doc = "Field `size` reader - "]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `size` writer - "]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_ADDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:31 - In the master mode, it is the value of address in \"address\" phase."]
    #[inline(always)]
    pub fn iodata_start_addr(&self) -> IODATA_START_ADDR_R {
        IODATA_START_ADDR_R::new(self.bits)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the master mode, it is the value of address in \"address\" phase."]
    #[inline(always)]
    pub fn iodata_start_addr(&mut self) -> IODATA_START_ADDR_W<0> {
        IODATA_START_ADDR_W::new(self)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<24> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In the master mode, it is the value of address in \"address\" phase.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_addr](index.html) module"]
pub struct SPI_ADDR_SPEC;
impl crate::RegisterSpec for SPI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_addr::R](R) reader structure"]
impl crate::Readable for SPI_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_addr::W](W) writer structure"]
impl crate::Writable for SPI_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_ADDR to value 0"]
impl crate::Resettable for SPI_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}