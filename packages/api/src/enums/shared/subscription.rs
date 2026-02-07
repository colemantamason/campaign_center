use crate::define_enum;

define_enum! {
    #[derive(Eq, Hash)]
    pub enum SubscriptionType {
        Events => ("events", "Events"),
    }
}
