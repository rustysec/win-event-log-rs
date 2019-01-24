use std::ffi::CString;
use std::fmt;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use widestring::U16String;
use winapi::shared::minwindef::{BOOL, DWORD, FILETIME, PBYTE, PDWORD};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
use winapi::um::winnt::{
    HANDLE, LCID, LONGLONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PHANDLE, PSID, PVOID, ULONGLONG,
};

#[allow(dead_code)]
const EvtQueryChannelPath: u32 = 0x1;
const EvtQueryFilePath: u32 = 0x2;
const EvtQueryForwardDirection: u32 = 0x100;
const EvtQueryReverseDirection: u32 = 0x200;
const EvtQueryTolerateQueryErrors: u32 = 0x1000;

type EvtHandle = HANDLE;
type PevtHandle = *mut HANDLE;

/// Defines the EvtQuery() function signature, for lazy loading
type EvtQueryFn = unsafe extern "system" fn(
    Session: EvtHandle,
    Path: LPCWSTR,
    Query: LPCWSTR,
    Flags: DWORD,
) -> EvtHandle;

/// Defines the EvtNext() function signature, for lazy loading
type EvtNextFn = unsafe extern "system" fn(
    ResultSet: EvtHandle,
    EventsSize: DWORD,
    Events: PevtHandle,
    Timeout: DWORD,
    Flags: DWORD,
    Returned: PDWORD,
) -> BOOL;

/// Defines the EvtRender() function signature, for lazy loading
type EvtRenderFn = unsafe extern "system" fn(
    Context: EvtHandle,
    Fragment: EvtHandle,
    Flags: DWORD,
    BufferSize: DWORD,
    Buffer: PVOID,
    BufferUsed: PDWORD,
    PropertyCount: PDWORD,
) -> BOOL;

/// Simply tries to dynamically load a function from `wevtapi.dll`, if the
/// function does not exist, or the DLL cannot be loaded, it returns `None`
///
/// # Arguments
/// `function` - the name of the function to load
///
fn try_load_from_dll<T>(function: &str) -> Option<Box<T>> {
    let module = CString::new("wevtapi.dll").unwrap().as_ptr();
    let function = CString::new(function).unwrap();

    let handle = match unsafe { GetModuleHandleA(module) } {
        i if i == null_mut() => match unsafe { LoadLibraryA(module) } {
            i if i == null_mut() => None,
            i => Some(i),
        },
        i => Some(i),
    };
    match handle {
        Some(h) => match unsafe { GetProcAddress(h, function.as_ptr()) } {
            i if i == null_mut() => None,
            addr => unsafe { Some(transmute::<HANDLE, Box<T>>(addr as _)) },
        },
        None => None,
    }
}

lazy_static! {
    pub static ref EvtQuery: Option<Box<EvtQueryFn>> =
        { try_load_from_dll::<EvtQueryFn>("EvtQuery") };
    pub static ref EvtNext: Option<Box<EvtNextFn>> = { try_load_from_dll::<EvtNextFn>("EvtQuery") };
    pub static ref EvtRender: Option<Box<EvtRenderFn>> =
        { try_load_from_dll::<EvtRenderFn>("EvtQuery") };
}

/// Entry point for querying the event log
pub struct WinEvents {
    handle: Option<EvtHandle>,
}

impl WinEvents {
    /// Queries the event log and returns a `Result`. If the APIs do not exist on
    /// this platform or there is an error performing the query, the `Error` will
    /// describe the problem.
    ///
    /// # Examples
    ///
    /// ```
    /// let query = QueryList::new();
    /// // add query information
    /// let events = WinEvents::query(query);
    /// ```
    ///
    /// ```
    /// // Free to use structured XML or XPath 1.0 as well!
    /// let events = WinEvents::query(r#"
    /// <QueryList>
    /// </QueryList>"#);
    /// ```
    ///
    pub fn get<T: Into<String>>(query: T) -> Result<WinEvents, String> {
        if EvtQuery.is_none() || EvtNext.is_none() || EvtRender.is_none() {
            Err("EvtQuery API is not available".to_owned())
        } else {
            let evt_query = EvtQuery.clone().unwrap();
            let ffi_query = U16String::from(query.into());
            match unsafe {
                evt_query(
                    null_mut(),
                    null_mut(),
                    ffi_query.as_ptr(),
                    EvtQueryChannelPath,
                )
            } {
                i if i == null_mut() => Err("There was an errorr processing the query".to_owned()),
                i => Ok(WinEvents { handle: Some(i) }),
            }
        }
    }
}

impl IntoIterator for WinEvents {
    type Item = Event;
    type IntoIter = WinEventsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        WinEventsIntoIterator { win_events: self }
    }
}

pub struct Event {
    inner: String,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

pub struct WinEventsIntoIterator {
    win_events: WinEvents,
}

impl Iterator for WinEventsIntoIterator {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_params_query_list() {
        use crate::{QueryList, WinEvents};
        let ql = QueryList::new();
        let we = WinEvents::get(ql);
        assert!(true);
    }

    #[test]
    fn test_params_str() {
        use crate::WinEvents;
        let we = WinEvents::get("test str");
        assert!(true);
    }
}
