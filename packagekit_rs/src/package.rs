use glib;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;
use std::fmt;

use package_kit_glib_sys::*;

glib::wrapper! {
    pub struct PackagePk(Interface<PkPackage>);

    match fn {
        type_ => || pk_package_get_type(),
    }
}

impl PackagePk {
    #[doc(alias = "pk_package_new")]
    pub fn new() -> PackagePk {
        unsafe { from_glib_full(pk_package_new()) }
    }

    pub fn ids_to_string(package_ids: *mut *mut c_char) -> glib::GString {
        unsafe { from_glib_full(pk_package_ids_to_string(package_ids)) }
    }
}

impl Default for PackagePk {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PackagePk {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&PackagePkExt::name(self))
    }
}

pub const NONE_PACKAGE: Option<&PackagePk> = None;

pub trait PackagePkExt: 'static {
    fn name(&self) -> String;
    fn id(&self) -> String;
    fn info(&self) -> String;
    fn summary(&self) -> String;
    fn version(&self) -> String;
    fn data(&self) -> String;
}

impl<O: IsA<PackagePk>> PackagePkExt for O {
    fn name(&self) -> String {
        unsafe {
            let s: glib::GString =
                from_glib_none(pk_package_get_name(self.as_ref().to_glib_none().0));
            s.to_string()
        }
    }

    fn id(&self) -> String {
        unsafe {
            let s: glib::GString =
                from_glib_none(pk_package_get_id(self.as_ref().to_glib_none().0));
            s.to_string()
        }
    }

    fn info(&self) -> String {
        unsafe {
            let info = pk_info_enum_to_string(pk_package_get_info(self.as_ref().to_glib_none().0));
            let s: glib::GString = from_glib_none(info);
            s.to_string()
        }
    }

    fn summary(&self) -> String {
        unsafe {
            let s: glib::GString =
                from_glib_none(pk_package_get_summary(self.as_ref().to_glib_none().0));
            s.to_string()
        }
    }

    fn version(&self) -> String {
        unsafe {
            let s: glib::GString =
                from_glib_none(pk_package_get_version(self.as_ref().to_glib_none().0));
            s.to_string()
        }
    }

    fn data(&self) -> String {
        unsafe {
            let s: glib::GString =
                from_glib_none(pk_package_get_data(self.as_ref().to_glib_none().0));
            s.to_string()
        }
    }
}
