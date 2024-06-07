#![allow(dead_code)]

use serde::Deserialize;

#[cfg(feature = "xml")]
#[cfg(feature = "xml")]
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct Provider {
    pub name: Option<String>,
    pub guid: Option<String>,
}

#[cfg(feature = "xml")]
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct System {
    pub provider: Option<Provider>,
}

#[cfg(feature = "xml")]
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
struct MyEvent {
    pub system: Option<System>,
}

#[cfg(all(feature = "xml", feature = "subscriber"))]
fn main() {
    use std::{thread::sleep, time::Duration};
    use win_event_log::prelude::*;

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

    match WinEventsSubscriber::get(query) {
        Ok(mut events) => {
            println!("Ctrl+C to quit!");
            while let Some(_event) = events.next() {
                // catch up to present
            }
            println!("Waiting for new events...");
            loop {
                while let Some(event) = events.next() {
                    let parsed: MyEvent = event.into();
                    println!("Parsed: {:?}", parsed);
                }
                sleep(Duration::from_millis(200));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(not(all(feature = "xml", feature = "subscriber")))]
fn main() {
    println!("This example requires serde");
}
