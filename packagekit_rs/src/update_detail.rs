use glib;
use glib::object::IsA;
use glib::translate::*;
//use libc::{c_char};

use package_kit_glib_sys::*;

glib::glib_wrapper! {
    pub struct UpdateDetailPk(Object<PkUpdateDetail, PkUpdateDetailClass, UpdateDetailClass>);

    match fn {
        get_type => || pk_update_detail_get_type(),
    }
}

impl UpdateDetailPk {
    #[doc(alias = "pk_update_detail_new")]
    pub fn new() -> UpdateDetailPk {
        unsafe { from_glib_full(pk_update_detail_new()) }
    }
}

impl Default for UpdateDetailPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_UPDATEDETAIL: Option<&UpdateDetailPk> = None;

pub trait UpdateDetailPkExt: 'static {
    /*
    #[doc(alias = "get_bugzilla_urls")]
    fn bugzilla_urls(&self) -> vec<glib::gstring>;

    #[doc(alias = "get_changelog")]
    fn changelog(&self) -> glib::gstring;

    #[doc(alias = "get_cve_urls")]
    fn cve_urls(&self) -> vec<glib::gstring>;

    #[doc(alias = "get_issued")]
    fn issued(&self) -> glib::gstring;

    */
    #[doc(alias = "get_obsoletes")]
    fn obsoletes(&self) -> Vec<glib::GString>;

    #[doc(alias = "get_package_id")]
    fn package_id(&self) -> glib::GString;

    #[doc(alias = "get_update_text")]
    fn update_text(&self) -> glib::GString;

    #[doc(alias = "get_updated")]
    fn updated(&self) -> glib::GString;

    #[doc(alias = "get_restart")]
    fn restart(&self) -> i32;

    //#[doc(alias = "get_updates")]
    //fn updates(&self) -> Vec<glib::GString>;

    //#[doc(alias = "get_vendor_urls")]
    //fn vendor_urls(&self) -> Vec<glib::GString>;
}

impl<O: IsA<UpdateDetailPk>> UpdateDetailPkExt for O {
    /*
    fn bugzilla_urls(&self) -> Vec<glib::GString> {
        unsafe {
            from_glib_none(pk_update_detail_get_bugzilla_urls(self.as_ref().to_glib_none().0))
        }
    }

    fn changelog(&self) -> glib::GString {
        unsafe {
            from_glib_none(pk_update_detail_get_changelog(self.as_ref().to_glib_none().0))
        }
    }

    fn cve_urls(&self) -> Vec<glib::GString> {
        unsafe {
            from_glib_none(pk_update_detail_get_cve_urls(self.as_ref().to_glib_none().0))
        }
    }

    fn issued(&self) -> glib::GString {
        unsafe {
            from_glib_none(pk_update_detail_get_issued(self.as_ref().to_glib_none().0))
        }
    }

    */
    fn obsoletes(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(pk_update_detail_get_obsoletes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn package_id(&self) -> glib::GString {
        unsafe {
            from_glib_none(pk_update_detail_get_package_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn update_text(&self) -> glib::GString {
        unsafe {
            from_glib_none(pk_update_detail_get_update_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn updated(&self) -> glib::GString {
        unsafe { from_glib_none(pk_update_detail_get_updated(self.as_ref().to_glib_none().0)) }
    }

    //fn restart(&self) -> PkRestartEnum {
    fn restart(&self) -> i32 {
        unsafe { pk_update_detail_get_restart(self.as_ref().to_glib_none().0) as i32 }
    }

    //fn updates(&self) -> Vec<glib::GString> {
    //unsafe {
    //from_glib_none(pk_update_detail_get_updates(self.as_ref().to_glib_none().0))
    //}
    //}

    //fn vendor_urls(&self) -> Vec<glib::GString> {
    //unsafe {
    //from_glib_none(pk_update_detail_get_vendor_urls(self.as_ref().to_glib_none().0))
    //}
    //}
}
