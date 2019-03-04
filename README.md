# win-event-log

A library for simple interface into the Windows Event Log system.

[![Build Status](https://travis-ci.org/rustysec/win-event-log-rs.svg?branch=master)](https://travis-ci.org/rustysec/win-event-log-rs)

## Usage
```toml
[dependencies]
win-event-log = { git = "https://github.com/rustysec/win-event-log-rs" }
```

This example will print the raw XML event from the event log:
```rust
extern crate win_event_log;

use win_event_log::prelude::*;

fn main() {
    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::Equal)),
        Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
    ];
    let query = QueryList::new()
        .with_query(
            Query::new()
                .item(
                    QueryItem::selector("Application".to_owned())
                        .system_conditions(Condition::or(conditions))
                        .build(),
                )
                .query(),
        )
        .build();

    match WinEvents::get(query) {
        Ok(events) => {
            if let Some(event) = events.into_iter().next() {
                println!("{}", event);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

The `examples/` folder has an example for easily deserializing the XML objects into a struct.

## Important
Currently, the Windows APIs used internally are only available on Vista+, meaning this library will
not be able to access events on legacy systems such as XP or Server 2003. However, the required methods
are loaded dynamically so executables compiled against this library will continue to work on these legacy
systems (`win-event-log` will just return `Error` results).
