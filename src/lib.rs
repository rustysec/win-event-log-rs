//! A simple crate for querying the Windows Event Log
//!
//! This library provides a mechanism for calling into the Windows Event subsystem.
//! It does use some unsafe code, but proper checks are done to ensure nothing explodes
//! at runtime. Currently, the API class used to query events is only present on Vista+,
//! so these functions are loaded dynamically instead of directly linked. This ensures
//! applications compiled against this crate will continue to work on legacy systems such
//! as XP or Server 2003. In the future, the legacy APIs will be integrated seamlessly
//! here to provide a consistent API surface.
//!
//! # Examples
//!
//! ```rust
//! extern crate win_event_log;
//! use win_event_log::prelude::*;
//!
//! let conditions = vec![ Condition::filter(EventFilter::level(1, Comparison::Equal)) ];
//! let query =
//! let query = QueryList.new()
//!     .with_query(
//!         Query::new().item(
//!             QueryItem::selector("System")
//!             .system_conditions(Conditions::or(conditions))
//!         ).query()
//!     ).build();
//! let events = WinEvent5::get(query).unwrap();
//! ```
//!

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "xml")]
extern crate serde;
#[cfg(feature = "xml")]
extern crate serde_derive;
#[cfg(feature = "xml")]
extern crate serde_xml_rs;
extern crate winapi;

mod api;
mod query_list;
#[cfg(feature = "subscriber")]
mod subscriber;
#[allow(unused_imports)]
use api::WinEvents;
#[allow(unused_imports)]
use query_list::QueryList;

pub mod prelude {
    pub use crate::api::*;
    pub use crate::query_list::*;
    #[cfg(feature = "subscriber")]
    pub use crate::subscriber::*;
}
