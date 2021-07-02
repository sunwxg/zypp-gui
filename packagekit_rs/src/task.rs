use glib;
use glib::object::IsA;
use glib::translate::*;

use package_kit_glib_sys::*;

glib::wrapper! {
    pub struct TaskPk(Interface<PkTask>);

    match fn {
        type_ => || pk_task_get_type(),
    }
}

impl TaskPk {
    #[doc(alias = "pk_task_new")]
    pub fn new() -> TaskPk {
        unsafe { from_glib_full(pk_task_new()) }
    }
}

impl Default for TaskPk {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TASK: Option<&TaskPk> = None;

pub trait PkTaskExt: 'static {
    #[doc(alias = "pk_task_install_packages_sync")]
    #[doc(alias = "pk_task_set_only_download")]
    fn set_only_download(&self, only_download: bool);

    fn set_simulate(&self, simulate: bool);
}

impl<O: IsA<TaskPk>> PkTaskExt for O {
    fn set_only_download(&self, only_download: bool) {
        unsafe {
            pk_task_set_only_download(self.as_ref().to_glib_none().0, only_download as i32);
        }
    }

    fn set_simulate(&self, simulate: bool) {
        unsafe {
            pk_task_set_simulate(self.as_ref().to_glib_none().0, simulate as i32);
        }
    }
}
