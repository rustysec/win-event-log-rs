use crate::api::{Event, EvtApi, EvtSubscribe, WinEvents, WinEventsIntoIterator};
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::um::handleapi::CloseHandle;
use winapi::um::synchapi::{CreateEventA, WaitForSingleObject};
use winapi::um::winbase::{INFINITE, WAIT_OBJECT_0};
use winapi::um::winevt::EvtSubscribeStartAtOldestRecord;
use winapi::um::winnt::HANDLE;

pub struct HandleWrapper(HANDLE);

impl Drop for HandleWrapper {
    fn drop(&mut self) {
        if self.0 != null_mut() {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}

pub struct WinEventsSubscriber {
    signal: Option<HandleWrapper>,
    events: Option<WinEventsIntoIterator>,
    has_events: bool,
}

impl WinEventsSubscriber {
    pub fn get<T: Into<String> + Clone>(query: T) -> Result<WinEventsSubscriber, String> {
        let ffi_query = {
            let mut tmp = OsString::from(query.into())
                .encode_wide()
                .collect::<Vec<u16>>();
            tmp.append(&mut OsString::from("\0").encode_wide().collect::<Vec<u16>>());
            tmp
        };
        let signal = unsafe { CreateEventA(null_mut(), 1, 1, null_mut()) };
        if let Some(EvtApi::Subscribe(ref evt_subscribe)) = *EvtSubscribe {
            let subscription = unsafe {
                evt_subscribe(
                    null_mut(),
                    signal,
                    null_mut(),
                    ffi_query.as_ptr(),
                    null_mut(),
                    null_mut(),
                    None,
                    EvtSubscribeStartAtOldestRecord,
                )
            };
            Ok(WinEventsSubscriber {
                signal: Some(HandleWrapper(signal)),
                events: Some(WinEvents::new(subscription).into_iter()),
                has_events: false,
            })
        } else {
            Err("There is an error calling the EvtSubscribe() API".to_owned())
        }
    }

    /// Gets the next item from the event log. If there are no more evens `None` is returned.
    pub fn next(&mut self) -> Option<Event> {
        match self.signal {
            Some(ref mut signal) => {
                if !self.has_events {
                    let x = unsafe { WaitForSingleObject(signal.0 as _, INFINITE) };
                    self.has_events = x == WAIT_OBJECT_0;
                }
                match self.has_events {
                    true => match self.events {
                        Some(ref mut events) => {
                            let event = events.next();
                            self.has_events = event.is_some();
                            event
                        }
                        None => {
                            self.has_events = false;
                            None
                        }
                    },
                    false => None,
                }
            }
            None => None,
        }
    }
}
