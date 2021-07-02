use glib;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;

use crate::details::DetailsPk;
use crate::package::PackagePk;
use crate::update_detail::UpdateDetailPk;
use package_kit_glib_sys::*;

glib::wrapper! {
    pub struct ResultsPk(Interface<PkResults>);

    match fn {
        type_ => || pk_results_get_type(),
    }
}

impl ResultsPk {
    #[doc(alias = "pk_results_new")]
    pub fn new() -> ResultsPk {
        unsafe { from_glib_full(pk_results_new()) }
    }
}

impl Default for ResultsPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_RESULTS: Option<&ResultsPk> = None;

pub trait ResultsPkExt: 'static {
    fn package_array(&self) -> Vec<PackagePk>;

    fn package_ids(&self) -> Option<*mut *mut c_char>;

    fn detail_array(&self) -> Vec<UpdateDetailPk>;

    fn require_restart_len(&self) -> i32;

    fn require_restart_worst(&self) -> i32;
}

impl<O: IsA<ResultsPk>> ResultsPkExt for O {
    fn package_array(&self) -> Vec<PackagePk> {
        unsafe {
            {
                let sack = pk_results_get_package_sack(self.as_ref().to_glib_none().0);
                pk_package_sack_sort(sack, PK_PACKAGE_SACK_SORT_TYPE_NAME);
                let array = pk_package_sack_get_array(sack);
                Vec::from_raw_parts(
                    (*array).pdata as *mut PackagePk,
                    (*array).len as usize,
                    (*array).len as usize,
                )
            }
        }
    }

    fn package_ids(&self) -> Option<*mut *mut c_char> {
        unsafe {
            let sack = pk_results_get_package_sack(self.as_ref().to_glib_none().0);
            if pk_package_sack_get_size(sack) == 0 {
                return None;
            }
            Some(pk_package_sack_get_ids(sack))
        }
    }

    fn detail_array(&self) -> Vec<UpdateDetailPk> {
        unsafe {
            {
                let array = pk_results_get_update_detail_array(self.as_ref().to_glib_none().0);
                Vec::from_raw_parts(
                    (*array).pdata as *mut UpdateDetailPk,
                    (*array).len as usize,
                    (*array).len as usize,
                )
            }
        }
    }

    fn require_restart_len(&self) -> i32 {
        unsafe {
            {
                let array = pk_results_get_require_restart_array(self.as_ref().to_glib_none().0);
                Vec::from_raw_parts(
                    (*array).pdata as *mut DetailsPk,
                    (*array).len as usize,
                    (*array).len as usize,
                )
                .len() as i32
            }
        }
    }

    fn require_restart_worst(&self) -> i32 {
        unsafe { pk_results_get_require_restart_worst(self.as_ref().to_glib_none().0) }
    }
}
