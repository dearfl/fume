use crate::quoted_number;

pub(crate) const INTERFACE: &str = "ISteamApps";

pub mod get_app_list;

// for get_app_list, we actually don't need this
// however let allow AppId to be quoted anyway
quoted_number!(AppId);
