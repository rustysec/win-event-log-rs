//! A simple crate for querying the Windows Event Log
//!
//! This library provides a mechanism for calling into the Windows Event subsystem.
//! It does use some unsafe code, but proper checks are done to ensure nothing explodes
//! at runtime. Currently, the API class used to query events is only present on Vista+,
//! so these functions are loaded dynamically instead of directly linked. This ensures
//! applications compiled against this crate will continue to work on legacy systems such
//! as XP or Server 2003. In the future, the legacy APIs will be integrated seamlessly
//! here to provide a consistent API surface.

#[macro_use]
extern crate lazy_static;
#[cfg(feature = "xml")]
extern crate serde;
#[cfg(feature = "xml")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "xml")]
extern crate serde_xml_rs;
extern crate widestring;
extern crate winapi;

mod api;
pub use self::api::{WinEvents, WinEventsIntoIterator};
mod query_list;
pub use self::query_list::*;
