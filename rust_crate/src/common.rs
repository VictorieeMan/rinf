use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

/// This contains a message from Dart.
/// Optionally, a custom binary called `blob` can also be included.
/// This type is generic, and the message
/// can be of any type declared in Protobuf.
pub struct DartSignal<T> {
    pub message: T,
    pub blob: Option<Vec<u8>>,
}

/// This is a mutable cell type that cannot be shared across threads.
pub type SimpleCell<T> = RefCell<Option<T>>;
/// This is a mutable cell type that can be shared across threads.
pub type SharedCell<T> = Arc<Mutex<SimpleCell<T>>>;
