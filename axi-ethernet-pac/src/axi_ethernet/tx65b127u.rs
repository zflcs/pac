#[doc = "Register `tx65b127u` reader"]
pub struct R(crate::R<TX65B127U_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX65B127U_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX65B127U_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX65B127U_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TX65B127U_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 65-127 bytes frames transmitted, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx65b127u](index.html) module"]
pub struct TX65B127U_SPEC;
impl crate::RegisterSpec for TX65B127U_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx65b127u::R](R) reader structure"]
impl crate::Readable for TX65B127U_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx65b127u to value 0"]
impl crate::Resettable for TX65B127U_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
