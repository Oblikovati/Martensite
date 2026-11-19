// SPDX-License-Identifier: GPL-2.0-only
//! #555: only the nonescaping fresh scalar producer can lend owned source authority.
use super::*;
#[derive(Clone, Copy, Default)]
pub(crate) enum SourceAccess<'a> {
    #[default]
    Live,
    Owned(&'a crate::sketch_region_prism::OwnedGraphAccess<'a>),
}
impl<'owner> SourceAccess<'owner> {
    pub(crate) fn owned_certificates<'a>(
        self,
        census: &'a BoundaryCensus,
    ) -> Option<DomainCertificates<'a>>
    where
        'owner: 'a,
    {
        match self {
            Self::Live => None,
            Self::Owned(_) => self.certificates(census),
        }
    }
    pub(crate) fn certificates<'a>(
        self,
        census: &'a BoundaryCensus,
    ) -> Option<DomainCertificates<'a>>
    where
        'owner: 'a,
    {
        let mut view = DomainCertificates::new(census)?;
        if let Self::Owned(owner) = self {
            view.validate_owned(owner);
        }
        Some(view)
    }
}

pub(crate) fn certificate_in<'a>(
    census: &'a BoundaryCensus,
    domain: &BoundaryDomain,
    view: Option<&DomainCertificates<'a>>,
) -> Option<martensite_tkmath::predicates::roots::bezier_pair::BezierPairCertificate<'a>> {
    match view {
        Some(view) => view.certificate(domain),
        None => super::domain_certificate(census, domain),
    }
}
