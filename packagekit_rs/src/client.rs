use glib;
use glib::object::IsA;
use glib::translate::*;
use libc::c_char;
use std::boxed::Box as Box_;
use std::ptr;

use crate::ProgressPk;
use crate::ResultsPk;
use package_kit_glib_sys::*;

glib::glib_wrapper! {
    pub struct ClientPk(Object<PkClient, PkClientClass, ClientClass>);

    match fn {
        get_type => || pk_client_get_type(),
    }
}

impl ClientPk {
    #[doc(alias = "pk_client_new")]
    pub fn new() -> ClientPk {
        unsafe { from_glib_full(pk_client_new()) }
    }
}

impl Default for ClientPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CLIENT: Option<&ClientPk> = None;

pub trait ClientPkExt: 'static {
    #[doc(alias = "pk_client_set_background")]
    fn set_background(&self, background: bool);

    #[doc(alias = "pk_client_set_cache_age")]
    fn set_cache_age(&self, cache_age: u32);

    #[doc(alias = "pk_client_set_interactive")]
    fn set_interactive(&self, interactive: bool);

    #[doc(alias = "pk_client_refresh_cache")]
    fn refresh_cache(&self, force: bool) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_get_updates")]
    fn get_updates(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;

    fn get_updates_async<Q: FnOnce(Result<ResultsPk, glib::Error>) + Send + 'static>(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
        callback_ready: Q,
    );

    #[doc(alias = "pk_client_get_packages")]
    fn get_packages(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_update_packages")]
    fn update_packages(
        &self,
        package_ids: *mut *mut c_char, //&[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
        only_download: bool,
    ) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_get_update_detail")]
    fn get_update_detail(
        &self,
        package_ids: *mut *mut c_char,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_search_names")]
    fn search_names(
        &self,
        values: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_install_packages")]
    fn install_packages(
        &self,
        package_ids: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;

    #[doc(alias = "pk_client_remove_packages")]
    fn remove_packages(
        &self,
        package_ids: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error>;
}

impl<O: IsA<ClientPk>> ClientPkExt for O {
    fn set_background(&self, background: bool) {
        unsafe {
            pk_client_set_background(self.as_ref().to_glib_none().0, background.to_glib());
        }
    }

    fn set_cache_age(&self, cache_age: u32) {
        unsafe {
            pk_client_set_cache_age(self.as_ref().to_glib_none().0, cache_age);
        }
    }

    fn set_interactive(&self, interactive: bool) {
        unsafe {
            pk_client_set_interactive(self.as_ref().to_glib_none().0, interactive.to_glib());
        }
    }

    fn refresh_cache(&self, force: bool) -> Result<ResultsPk, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = pk_client_refresh_cache(
                self.as_ref().to_glib_none().0,
                force.to_glib(),
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn updates(&self) -> Result<ResultsPk, glib::Error> {
    //unsafe {
    //let flag = pk_transaction_flag_enum_to_string(0);
    //let filters = pk_transaction_flag_bitfield_from_string(flag);
    //let mut error = ptr::null_mut();
    //let ret = pk_client_get_updates(
    //self.as_ref().to_glib_none().0,
    //filters,
    //ptr::null_mut(),
    //None,
    //ptr::null_mut(),
    //&mut error);

    //if error.is_null() {
    //Ok(from_glib_full(ret))
    //} else {
    //Err(from_glib_full(error))
    //}
    //}
    //}

    fn get_updates(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;
        unsafe {
            let filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_NONE, -1);
            let mut error = ptr::null_mut();
            let ret = pk_client_get_updates(
                self.as_ref().to_glib_none().0,
                filters,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_updates_async<Q: FnOnce(Result<ResultsPk, glib::Error>) + Send + 'static>(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
        callback_ready: Q,
    ) {
        // Prepare progress callback
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        // Prepare callback_ready
        let user_data: Box_<Q> = Box_::new(callback_ready);
        unsafe extern "C" fn get_updates_async_trampoline<
            Q: FnOnce(Result<ResultsPk, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::object::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = pk_client_generic_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback_ready = get_updates_async_trampoline::<Q>;

        unsafe {
            let filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_NONE, -1);
            pk_client_get_updates_async(
                self.as_ref().to_glib_none().0,
                filters,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                Some(callback_ready),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn get_packages(
        &self,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let filters = pk_bitfield_from_enums(PK_FILTER_ENUM_INSTALLED, PK_FILTER_ENUM_ARCH, -1);
            let mut error = ptr::null_mut();
            let ret = pk_client_get_packages(
                self.as_ref().to_glib_none().0,
                filters,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn update_packages(
        &self,
        package_ids: *mut *mut c_char,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
        only_download: bool,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let filters: PkBitfield;
            if only_download {
                filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_ONLY_DOWNLOAD, -1);
            } else {
                filters = pk_bitfield_from_enums(
                    PK_TRANSACTION_FLAG_ENUM_NONE,
                    PK_TRANSACTION_FLAG_ENUM_ONLY_TRUSTED,
                    -1,
                );
            }
            let mut error = ptr::null_mut();

            let ret = pk_client_update_packages(
                self.as_ref().to_glib_none().0,
                filters,
                package_ids,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_update_detail(
        &self,
        package_ids: *mut *mut c_char,
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let mut error = ptr::null_mut();

            let ret = pk_client_get_update_detail(
                self.as_ref().to_glib_none().0,
                package_ids,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn search_names(
        &self,
        values: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let mut error = ptr::null_mut();
            let filters: PkBitfield;
            filters =
                pk_bitfield_from_enums(PK_FILTER_ENUM_ARCH, PK_FILTER_ENUM_NOT_COLLECTIONS, -1);

            let ret = pk_client_search_names(
                self.as_ref().to_glib_none().0,
                filters,
                values.to_glib_none().0,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn install_packages(
        &self,
        package_ids: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let mut error = ptr::null_mut();
            let filters: PkBitfield;
            filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_ONLY_TRUSTED, -1);
            //filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_NONE,
            //PK_TRANSACTION_FLAG_ENUM_ONLY_TRUSTED,
            //-1);

            let ret = pk_client_install_packages(
                self.as_ref().to_glib_none().0,
                filters,
                package_ids.to_glib_none().0,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_packages(
        &self,
        package_ids: &[&str],
        progress_callback: Option<Box_<dyn (Fn(&ProgressPk, PkProgressType)) + 'static>>,
    ) -> Result<ResultsPk, glib::Error> {
        let progress_callback_data: Box_<
            Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>,
        > = Box_::new(progress_callback);
        unsafe extern "C" fn progress_callback_func(
            progress: *mut PkProgress,
            type_: PkProgressType,
            user_data: glib_sys::gpointer,
        ) {
            let progress = from_glib_borrow(progress);
            let type_progress = type_;
            let callback: &Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>> =
                &*(user_data as *mut _);

            let _res = if let Some(ref callback) = *callback {
                callback(&progress, type_progress)
            } else {
                panic!("cannot get closure...")
            };
        }

        let progress_callback = if progress_callback_data.is_some() {
            Some(progress_callback_func as _)
        } else {
            None
        };

        let super_callback0: Box_<Option<Box_<dyn Fn(&ProgressPk, PkProgressType) + 'static>>> =
            progress_callback_data;

        unsafe {
            let mut error = ptr::null_mut();
            let filters: PkBitfield;
            //filters = pk_bitfield_from_enums(PK_TRANSACTION_FLAG_ENUM_ONLY_TRUSTED, -1);
            filters = pk_bitfield_from_enums(
                PK_TRANSACTION_FLAG_ENUM_NONE,
                PK_TRANSACTION_FLAG_ENUM_ONLY_TRUSTED,
                -1,
            );

            let ret = pk_client_remove_packages(
                self.as_ref().to_glib_none().0,
                filters,
                package_ids.to_glib_none().0,
                1,
                1,
                ptr::null_mut(),
                progress_callback,
                Box_::into_raw(super_callback0) as *mut _,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
