use std::sync::Arc;
use atri_plugin::event::GroupMessageEvent;
use atri_plugin::listener::{ListenerBuilder, ListenerGuard, Priority};
use atri_plugin::Plugin;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;

struct AtriBotctl {
    client_group: Arc<DashMap<i64, i64>>,
    listener: Option<ListenerGuard>,
}

impl Plugin for AtriBotctl {
    fn new() -> Self {
        Self {
            client_group: Arc::new(DashMap::new()),
            listener: None
        }
    }

    fn enable(&mut self) {
        let client_group = self.client_group.clone();

        self.listener = Some(ListenerBuilder::listening_on_always(move |e: GroupMessageEvent| {
            let client_group = client_group.clone();
            let client_id = e.client().id();
            let group_id = e.group().id();

            async move {
                match client_group.entry(group_id) {
                    Entry::Occupied(has) => {
                        if client_id != *has.get() {
                            e.intercept();
                        }
                    }
                    Entry::Vacant(hasn) => {
                        hasn.insert(client_id);
                    }
                }
            }
        })
            .priority(Priority::Top)
            .start()
        );
    }
}