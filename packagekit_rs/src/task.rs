use glib;
use glib::object::IsA;
use glib::translate::*;
//use std::{ptr};

use crate::ClientPk;
use package_kit_glib_sys::*;

glib::glib_wrapper! {
    pub struct Task(Object<PkTask, PkTaskClass, TaskClass>) @extends ClientPk;
    //pub struct Task(Object<PkTask, PkTaskClass, TaskClass>);

    match fn {
        get_type => || pk_task_get_type(),
    }
}

impl Task {
    #[doc(alias = "pk_task_new")]
    pub fn new() -> Task {
        unsafe { from_glib_full(pk_task_new()) }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TASK: Option<&Task> = None;

pub trait PkTaskExt: 'static {
    #[doc(alias = "pk_task_install_packages_sync")]
    #[doc(alias = "pk_task_set_only_download")]
    fn set_only_download(&self, only_download: bool);

    fn set_simulate(&self, simulate: bool);
    //fn update_packages_sync(&self, package_ids: &[&str]);
}

impl<O: IsA<Task>> PkTaskExt for O {
    fn set_only_download(&self, only_download: bool) {
        unsafe {
            pk_task_set_only_download(self.as_ref().to_glib_none().0, only_download.to_glib());
        }
    }

    fn set_simulate(&self, simulate: bool) {
        unsafe {
            pk_task_set_simulate(self.as_ref().to_glib_none().0, simulate.to_glib());
        }
    }
    /*
    fn update_packages_sync(&self, package_ids: &[&str]) {
        unsafe {
            //let mut error = ptr::null_mut();
            //let ret = pk_task_update_packages_sync(
            pk_task_update_packages_sync(
                self.as_ref().to_glib_none().0,
                package_ids.to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                ptr::null_mut());
            //println!("{:?}", *ret);
        }
    }
    */
}
