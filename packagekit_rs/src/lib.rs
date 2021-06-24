pub mod prelude;

pub mod results;
pub use self::results::ResultsPkExt;
pub use self::results::{ResultsClass, ResultsPk, NONE_RESULTS};

pub mod package;
pub use self::package::PackagePkExt;
pub use self::package::{PackageClass, PackagePk, NONE_PACKAGE};

pub mod details;
pub use self::details::DetailsPkExt;
pub use self::details::{DetailsClass, DetailsPk, NONE_DETAILS};

pub mod update_detail;
pub use self::update_detail::UpdateDetailPkExt;
pub use self::update_detail::{UpdateDetailClass, UpdateDetailPk, NONE_UPDATEDETAIL};

pub mod progress;
pub use self::progress::ProgressPkExt;
pub use self::progress::{ProgressClass, ProgressPk, NONE_PROGRESS};

pub mod client;
pub use self::client::ClientPkExt;
pub use self::client::{ClientClass, ClientPk, NONE_CLIENT};

pub mod task;
pub use self::task::PkTaskExt;
pub use self::task::{Task, TaskClass, NONE_TASK};

//pub mod
pub use package_kit_glib_sys::{
    pk_offline_get_prepared_ids, pk_offline_trigger, PK_OFFLINE_ACTION_REBOOT,
    PK_PROGRESS_TYPE_PERCENTAGE, PK_STATUS_ENUM_CLEANUP, PK_STATUS_ENUM_DOWNLOAD,
    PK_STATUS_ENUM_INSTALL, PK_STATUS_ENUM_REFRESH_CACHE, PK_STATUS_ENUM_REMOVE,
    PK_STATUS_ENUM_UPDATE,
};

extern crate gio_sys;
extern crate glib;

//mod client;
//mod task;
//pub use self::client::*;
//pub use self::task::*;

//mod auto;

//pub use auto::*;
//use auto::Client;

#[cfg(test)]
mod tests {

    //#[test]
    //fn refresh_cache() {
    //let client = Client.new();
    //}
}
