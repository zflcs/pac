#[doc = "Register `dlh` reader"]
pub struct R(crate::R<DLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dlh` writer"]
pub struct W(crate::W<DLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLH_SPEC>;
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
impl From<crate::W<DLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dlh` reader - "]
pub type DLH_R = crate::FieldReader;
#[doc = "Field `dlh` writer - "]
pub type DLH_W<'a, const O: u8> = crate::FieldWriter<'a, DLH_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dlh(&self) -> DLH_R {
        DLH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dlh(&mut self) -> DLH_W<0> {
        DLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Divisor Latch High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlh](index.html) module"]
pub struct DLH_SPEC;
impl crate::RegisterSpec for DLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlh::R](R) reader structure"]
impl crate::Readable for DLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlh::W](W) writer structure"]
impl crate::Writable for DLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dlh to value 0"]
impl crate::Resettable for DLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}