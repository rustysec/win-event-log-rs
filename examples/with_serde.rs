extern crate win_event_log;
#[macro_use]
#[cfg(feature = "xml")]
extern crate serde_derive;

#[cfg(feature = "xml")]
use win_event_log::{Comparison, Condition, EventFilter, Query, QueryList, Selector, WinEvents};

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

#[cfg(feature = "xml")]
fn main() {
    let conditions = vec![
        Condition::filter(EventFilter::level(1, Comparison::Equal)),
        Condition::filter(EventFilter::level(4, Comparison::GreaterThanOrEqual)),
    ];
    let query = QueryList::new()
        .with_query(
            Query::new()
                .select(
                    Selector::new("Application".to_owned())
                        .system_conditions(Condition::or(conditions))
                        .build(),
                )
                .query(),
        )
        .build();

    match WinEvents::get(query) {
        Ok(events) => {
            if let Some(event) = events.into_iter().next() {
                let parsed: MyEvent = event.into();
                println!("Parsed: {:?}", parsed);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(not(feature = "xml"))]
fn main() {
    println!("This example requires serde");
}
