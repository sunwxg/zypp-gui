use glib;
use glib::object::IsA;
use glib::translate::*;

use crate::package::PackagePk;
use package_kit_glib_sys::*;

glib::wrapper! {
    pub struct ProgressPk(Interface<PkProgress>);

    match fn {
        type_ => || pk_progress_get_type(),
    }
}

impl ProgressPk {
    #[doc(alias = "pk_progress_new")]
    pub fn new() -> ProgressPk {
        unsafe { from_glib_full(pk_progress_new()) }
    }
}

impl Default for ProgressPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_PROGRESS: Option<&ProgressPk> = None;

pub trait ProgressPkExt: 'static {
    fn package(&self) -> PackagePk;

    fn package_id(&self) -> glib::GString;

    fn percentage(&self) -> i32;

    fn status(&self) -> i32;

    fn transaction_id(&self) -> glib::GString;

    fn get_role(&self) -> glib::GString;

    fn get_item_package(&self) -> String;
}

impl<O: IsA<ProgressPk>> ProgressPkExt for O {
    fn package(&self) -> PackagePk {
        unsafe { from_glib_none(pk_progress_get_package(self.as_ref().to_glib_none().0)) }
    }

    fn package_id(&self) -> glib::GString {
        unsafe { from_glib_none(pk_progress_get_package_id(self.as_ref().to_glib_none().0)) }
    }

    fn percentage(&self) -> i32 {
        unsafe { pk_progress_get_percentage(self.as_ref().to_glib_none().0) }
    }

    fn status(&self) -> i32 {
        unsafe { pk_progress_get_status(self.as_ref().to_glib_none().0) }
    }

    fn transaction_id(&self) -> glib::GString {
        unsafe {
            from_glib_none(pk_progress_get_transaction_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_role(&self) -> glib::GString {
        unsafe {
            let role = pk_progress_get_role(self.as_ref().to_glib_none().0);
            from_glib_none(pk_role_enum_to_string(role))
        }
    }

    fn get_item_package(&self) -> String {
        unsafe {
            let item_progress = pk_progress_get_item_progress(self.as_ref().to_glib_none().0);
            let id = pk_item_progress_get_package_id(item_progress);
            let s: glib::GString = from_glib_none(id);
            s.to_string()
        }
    }
}
