import 'dart:typed_data';

/// This type represents a function
/// that can accept raw signal data from Rust
/// and handle it accordingly.
typedef HandleSignal = void Function(int, Uint8List, Uint8List?);

/// This contains a message from Rust.
/// Optionally, a custom binary called `blob` can also be included.
/// This type is generic, and the message
/// can be of any type declared in Protobuf.
class RustSignal<T> {
  T message;
  Uint8List? blob;
  RustSignal(this.message, this.blob);
}
