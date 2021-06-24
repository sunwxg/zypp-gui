use glib;
use glib::object::IsA;
use glib::translate::*;

use package_kit_glib_sys::*;

glib::glib_wrapper! {
    pub struct DetailsPk(Object<PkDetails, PkDetailsClass, DetailsClass>);

    match fn {
        get_type => || pk_details_get_type(),
    }
}

impl DetailsPk {
    #[doc(alias = "pk_details_new")]
    pub fn new() -> DetailsPk {
        unsafe { from_glib_full(pk_details_new()) }
    }
}

impl Default for DetailsPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DETAILS: Option<&DetailsPk> = None;

pub trait DetailsPkExt: 'static {
    #[doc(alias = "pk_details_get_description")]
    fn description(&self) -> glib::GString;

    #[doc(alias = "pk_details_get_license")]
    fn license(&self) -> glib::GString;

    #[doc(alias = "pk_details_get_package_id")]
    fn package_id(&self) -> glib::GString;

    #[doc(alias = "pk_details_get_size")]
    fn size(&self) -> u64;

    #[doc(alias = "pk_details_get_summary")]
    fn summary(&self) -> glib::GString;

    #[doc(alias = "pk_details_get_url")]
    fn url(&self) -> glib::GString;
}

impl<O: IsA<DetailsPk>> DetailsPkExt for O {
    fn description(&self) -> glib::GString {
        unsafe { from_glib_none(pk_details_get_description(self.as_ref().to_glib_none().0)) }
    }

    fn license(&self) -> glib::GString {
        unsafe { from_glib_none(pk_details_get_license(self.as_ref().to_glib_none().0)) }
    }

    fn package_id(&self) -> glib::GString {
        unsafe { from_glib_none(pk_details_get_package_id(self.as_ref().to_glib_none().0)) }
    }

    fn size(&self) -> u64 {
        unsafe { pk_details_get_size(self.as_ref().to_glib_none().0) }
    }

    fn summary(&self) -> glib::GString {
        unsafe { from_glib_none(pk_details_get_summary(self.as_ref().to_glib_none().0)) }
    }

    fn url(&self) -> glib::GString {
        unsafe { from_glib_none(pk_details_get_url(self.as_ref().to_glib_none().0)) }
    }
}
