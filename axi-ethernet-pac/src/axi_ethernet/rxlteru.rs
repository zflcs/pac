#[doc = "Register `rxlteru` reader"]
pub struct R(crate::R<RXLTERU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLTERU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLTERU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLTERU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXLTERU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of frames received with length error, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlteru](index.html) module"]
pub struct RXLTERU_SPEC;
impl crate::RegisterSpec for RXLTERU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlteru::R](R) reader structure"]
impl crate::Readable for RXLTERU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxlteru to value 0"]
impl crate::Resettable for RXLTERU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
