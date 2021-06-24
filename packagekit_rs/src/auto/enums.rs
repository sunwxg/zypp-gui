//use glib::error::ErrorDomain;
use glib::translate::*;
//use glib::value::FromValue;
//use glib::value::FromValueOptional;
//use glib::value::SetValue;
//use glib::value::Value;
//use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::fmt;
use package_kit_glib_sys::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "PkProgressType")]
pub enum ProgressType {
    #[doc(alias = "PK_PROGRESS_TYPE_PACKAGE_ID")]
    PackageId,
    #[doc(alias = "PK_PROGRESS_TYPE_TRANSACTION_ID")]
    TransactionId,
    #[doc(alias = "PK_PROGRESS_TYPE_PERCENTAGE")]
    Percentage,
    #[doc(alias = "PK_PROGRESS_TYPE_ALLOW_CANCEL")]
    AllowCancel,
    #[doc(alias = "PK_PROGRESS_TYPE_STATUS")]
    Status,
    #[doc(alias = "PK_PROGRESS_TYPE_ROLE")]
    Role,
    #[doc(alias = "PK_PROGRESS_TYPE_CALLER_ACTIVE")]
    CallerActive,
    #[doc(alias = "PK_PROGRESS_TYPE_ELAPSED_TIME")]
    ElapsedTime,
    #[doc(alias = "PK_PROGRESS_TYPE_REMAINING_TIME")]
    RemainingTime,
    #[doc(alias = "PK_PROGRESS_TYPE_SPEED")]
    Speed,
    #[doc(alias = "PK_PROGRESS_TYPE_DOWNLOAD_SIZE_REMAINING")]
    DownloadSizeRemaining,
    #[doc(alias = "PK_PROGRESS_TYPE_UID")]
    Uid,
    #[doc(alias = "PK_PROGRESS_TYPE_PACKAGE")]
    Package,
    #[doc(alias = "PK_PROGRESS_TYPE_ITEM_PROGRESS")]
    ItemProgress,
    #[doc(alias = "PK_PROGRESS_TYPE_TRANSACTION_FLAGS")]
    TransactionFlags,
    #[doc(alias = "PK_PROGRESS_TYPE_INVALID")]
    Invalid,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ProgressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProgressType::{}", match *self {
            Self::PackageId => "PackageId",
            Self::TransactionId => "TransactionId",
            Self::Percentage => "Percentage",
            Self::AllowCancel => "AllowCancel",
            Self::Status => "Status",
            Self::Role => "Role",
            Self::CallerActive => "CallerActive",
            Self::ElapsedTime => "ElapsedTime",
            Self::RemainingTime => "RemainingTime",
            Self::Speed => "Speed",
            Self::DownloadSizeRemaining => "DownloadSizeRemaining",
            Self::Uid => "Uid",
            Self::Package => "Package",
            Self::ItemProgress => "ItemProgress",
            Self::TransactionFlags => "TransactionFlags",
            Self::Invalid => "Invalid",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for ProgressType {
    type GlibType = PkProgressType;

    fn to_glib(&self) -> PkProgressType {
        match *self {
            Self::PackageId => PK_PROGRESS_TYPE_PACKAGE_ID,
            Self::TransactionId => PK_PROGRESS_TYPE_TRANSACTION_ID,
            Self::Percentage => PK_PROGRESS_TYPE_PERCENTAGE,
            Self::AllowCancel => PK_PROGRESS_TYPE_ALLOW_CANCEL,
            Self::Status => PK_PROGRESS_TYPE_STATUS,
            Self::Role => PK_PROGRESS_TYPE_ROLE,
            Self::CallerActive => PK_PROGRESS_TYPE_CALLER_ACTIVE,
            Self::ElapsedTime => PK_PROGRESS_TYPE_ELAPSED_TIME,
            Self::RemainingTime => PK_PROGRESS_TYPE_REMAINING_TIME,
            Self::Speed => PK_PROGRESS_TYPE_SPEED,
            Self::DownloadSizeRemaining => PK_PROGRESS_TYPE_DOWNLOAD_SIZE_REMAINING,
            Self::Uid => PK_PROGRESS_TYPE_UID,
            Self::Package => PK_PROGRESS_TYPE_PACKAGE,
            Self::ItemProgress => PK_PROGRESS_TYPE_ITEM_PROGRESS,
            Self::TransactionFlags => PK_PROGRESS_TYPE_TRANSACTION_FLAGS,
            Self::Invalid => PK_PROGRESS_TYPE_INVALID,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<PkProgressType> for ProgressType {
    fn from_glib(value: PkProgressType) -> Self {
        //skip_assert_initialized!();
        match value {
            0 => Self::PackageId,
            1 => Self::TransactionId,
            2 => Self::Percentage,
            3 => Self::AllowCancel,
            4 => Self::Status,
            5 => Self::Role,
            6 => Self::CallerActive,
            7 => Self::ElapsedTime,
            8 => Self::RemainingTime,
            9 => Self::Speed,
            10 => Self::DownloadSizeRemaining,
            11 => Self::Uid,
            12 => Self::Package,
            13 => Self::ItemProgress,
            14 => Self::TransactionFlags,
            15 => Self::Invalid,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for ProgressType {
    fn static_type() -> Type {
        unsafe { from_glib(pk_progress_type_get_type()) }
    }
}
