use glib;
use glib::object::IsA;
use glib::translate::*;

use package_kit_glib_sys::*;

glib::wrapper! {
    pub struct UpdateDetailPk(Interface<PkUpdateDetail>);

    match fn {
        type_ => || pk_update_detail_get_type(),
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
}

impl<O: IsA<UpdateDetailPk>> UpdateDetailPkExt for O {
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

    fn restart(&self) -> i32 {
        unsafe { pk_update_detail_get_restart(self.as_ref().to_glib_none().0) as i32 }
    }
}
