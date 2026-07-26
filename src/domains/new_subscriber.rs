use crate::domains::SubscriberEmail;
use crate::domains::SubscriberName;

pub struct NewSubscriber {
    // We are not using `String` anymore!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
