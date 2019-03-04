#![allow(non_upper_case_globals)]

#[cfg(feature = "xml")]
use serde::Deserialize;
#[cfg(feature = "xml")]
use serde_xml_rs::from_str;
use std::ffi::{CString, OsString};
use std::fmt;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::prelude::*;
use std::ptr::null_mut;
use winapi::shared::minwindef::{BOOL, DWORD, PDWORD};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
use winapi::um::winevt::EVT_SUBSCRIBE_CALLBACK;
use winapi::um::winnt::{HANDLE, LPCWSTR, PVOID};

bitflags! {
    struct EvtQueryOptions: u32 {
        const EvtQueryChannelPath= 0x1;
        const EvtQueryFilePath= 0x2;
        const EvtQueryForwardDirection= 0x100;
        const EvtQueryReverseDirection= 0x200;
        const EvtQueryTolerateQueryErrors= 0x1000;
    }
}

pub type EvtHandle = HANDLE;
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

/// Defines the EvtClose() function signature, for lazy loading
type EvtCloseFn = unsafe extern "system" fn(Object: EvtHandle) -> BOOL;

/// Defines the EvtSubscribe() function signature, for lazy loading
type EvtSubscribeFn = unsafe extern "system" fn(
    Session: EvtHandle,
    SignalEvent: HANDLE,
    ChannelPath: LPCWSTR,
    Query: LPCWSTR,
    Bookmark: EvtHandle,
    Context: PVOID,
    Callback: EVT_SUBSCRIBE_CALLBACK,
    Flags: DWORD,
) -> EvtHandle;

#[derive(Clone)]
pub enum EvtApi {
    Close(EvtCloseFn),
    Next(EvtNextFn),
    Query(EvtQueryFn),
    Render(EvtRenderFn),
    Subscribe(EvtSubscribeFn),
}

/// Simply tries to dynamically load a function from `wevtapi.dll`, if the
/// function does not exist, or the DLL cannot be loaded, it returns `None`
///
/// # Arguments
/// `function` - the name of the function to load
///
fn try_load_from_dll(function: &str) -> Option<EvtApi> {
    let ffi_module = CString::new("wevtapi.dll").unwrap();
    let ffi_function = CString::new(function).unwrap();

    let handle = match unsafe { GetModuleHandleA(ffi_module.as_ptr()) } {
        i if i == null_mut() => match unsafe { LoadLibraryA(ffi_module.as_ptr()) } {
            j if j == null_mut() => None,
            j => Some(j),
        },
        i => Some(i),
    };
    match handle {
        Some(h) => match unsafe { GetProcAddress(h, ffi_function.as_ptr()) } {
            i if i == null_mut() => None,
            addr => match function.as_ref() {
                "EvtClose" => Some(EvtApi::Close(unsafe {
                    transmute::<HANDLE, EvtCloseFn>(addr as _)
                })),
                "EvtNext" => Some(EvtApi::Next(unsafe {
                    transmute::<HANDLE, EvtNextFn>(addr as _)
                })),
                "EvtQuery" => Some(EvtApi::Query(unsafe {
                    transmute::<HANDLE, EvtQueryFn>(addr as _)
                })),
                "EvtRender" => Some(EvtApi::Render(unsafe {
                    transmute::<HANDLE, EvtRenderFn>(addr as _)
                })),
                "EvtSubscribe" => Some(EvtApi::Subscribe(unsafe {
                    transmute::<HANDLE, EvtSubscribeFn>(addr as _)
                })),
                _ => None,
            },
        },
        None => None,
    }
}

lazy_static! {
    pub static ref EvtClose: Option<EvtApi> = { try_load_from_dll("EvtClose") };
    pub static ref EvtNext: Option<EvtApi> = { try_load_from_dll("EvtNext") };
    pub static ref EvtQuery: Option<EvtApi> = { try_load_from_dll("EvtQuery") };
    pub static ref EvtRender: Option<EvtApi> = { try_load_from_dll("EvtRender") };
    pub static ref EvtSubscribe: Option<EvtApi> = { try_load_from_dll("EvtSubscribe") };
}

pub struct EvtHandleWrapper(pub EvtHandle);

impl Drop for EvtHandleWrapper {
    fn drop(&mut self) {
        if let Some(EvtApi::Close(ref close)) = *EvtClose {
            unsafe {
                close(self.0);
            }
        }
    }
}

/// Entry point for querying the event log
pub struct WinEvents {
    handle: Option<EvtHandleWrapper>,
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
    pub fn get<T: Into<String> + Clone>(query: T) -> Result<WinEvents, String> {
        if EvtQuery.is_none() || EvtNext.is_none() || EvtRender.is_none() {
            Err("EvtQuery API is not available".to_owned())
        } else {
            // Small hack to prevent occasional parsing errors from the Evt* API
            let ffi_query = {
                let mut tmp = OsString::from(query.into())
                    .encode_wide()
                    .collect::<Vec<u16>>();
                tmp.append(&mut OsString::from("\0").encode_wide().collect::<Vec<u16>>());
                tmp
            };
            if let Some(EvtApi::Query(ref evt_query)) = *EvtQuery {
                match unsafe {
                    evt_query(
                        null_mut(),
                        null_mut(),
                        ffi_query.as_ptr(),
                        EvtQueryOptions::EvtQueryChannelPath.bits(),
                    )
                } {
                    i if i == null_mut() => Err(format!(
                        "There was an error processing the query: {}",
                        unsafe { GetLastError() }
                    )),
                    i => Ok(WinEvents {
                        handle: Some(EvtHandleWrapper(i)),
                    }),
                }
            } else {
                Err("There is an error calling EvtQuery() API".to_owned())
            }
        }
    }

    /// Gets the next item from the event log. If there are no more evens `None` is returned.
    pub fn next(&mut self) -> Option<EvtHandleWrapper> {
        if let Some(ref handle) = self.handle {
            let mut next_handle: Vec<EvtHandle> = vec![null_mut() as _];
            match *EvtNext {
                Some(EvtApi::Next(ref next)) => {
                    let mut number_returned: DWORD = 0;
                    if unsafe {
                        next(
                            handle.0,
                            1,
                            next_handle.as_mut_ptr() as _,
                            0,
                            0,
                            &mut number_returned,
                        )
                    } > 0
                    {
                        Some(EvtHandleWrapper(next_handle[0]))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Create a `WinEvents` from existing event handle
    pub fn new(handle: EvtHandle) -> WinEvents {
        WinEvents {
            handle: Some(EvtHandleWrapper(handle)),
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

pub struct Event(String);

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "xml")]
impl Event {
    pub fn into<'de, T>(self) -> T
    where
        T: Deserialize<'de> + Default,
    {
        match from_str(&self.0) {
            Ok(o) => o,
            _ => Default::default(),
        }
    }
}

/// An iterator abstraction over `WinEvents`
///
/// # Example
///
/// ```rust
/// let query = QueryList::new()
///     .with_query(Query::new().query())
///     .build();
/// let events = WinEvents::get(query).unwrap().into_iter();
/// while let Some(event) = events.next() {
/// // ...
/// }
/// ```
pub struct WinEventsIntoIterator {
    win_events: WinEvents,
}

impl Iterator for WinEventsIntoIterator {
    type Item = Event;

    /// Returns the next event. If there are no more events, `None` is returned.
    fn next(&mut self) -> Option<Event> {
        match self.win_events.next() {
            Some(handle) => match *EvtRender {
                Some(EvtApi::Render(ref render)) => {
                    let mut buffer_used: DWORD = 0;
                    let mut property_count: DWORD = 0;
                    unsafe {
                        if render(
                            null_mut(),
                            handle.0 as _,
                            1,
                            0,
                            null_mut(),
                            &mut buffer_used,
                            &mut property_count,
                        ) == 0
                            && GetLastError() == 122
                        {
                            let mut buf: Vec<u16> = vec![0; buffer_used as usize];
                            match render(
                                null_mut(),
                                handle.0 as _,
                                1,
                                buf.len() as _,
                                buf.as_mut_ptr() as _,
                                &mut buffer_used,
                                &mut property_count,
                            ) {
                                0 => None,
                                _ => {
                                    let s =
                                        OsString::from_wide(&buf[..]).to_string_lossy().to_string();
                                    Some(Event(s))
                                }
                            }
                        } else {
                            None
                        }
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_params_query_list() {
        use crate::{QueryList, WinEvents};
        use winapi::um::errhandlingapi::GetLastError;
        let ql = QueryList::new();
        match WinEvents::get(ql) {
            Ok(_) => assert!(true),
            Err(e) => match unsafe { GetLastError() } {
                // false positive from wine
                0 => assert!(true),
                _ => {
                    println!("test_params_query_list(): {}", e);
                    assert!(false);
                }
            },
        }
    }

    #[test]
    fn test_params_str() {
        use crate::WinEvents;
        use winapi::um::errhandlingapi::GetLastError;
        let query = r#"<QueryList>
<Query Id="0">
<Select Path="Security">
*[System[((Level = 0) or (Level >= 4))]]
and
*[EventData[((Data[@Name = 'TargetUserName']) and (Data = 'SYSTEM'))]]
</Select>
</Query>
</QueryList>"#;

        match WinEvents::get(query) {
            Ok(_) => assert!(true),
            Err(e) => match unsafe { GetLastError() } {
                // false positive from wine
                0 => assert!(true),
                _ => {
                    println!("test_params_str(): {}", e);
                    assert!(false);
                }
            },
        }
    }
}
