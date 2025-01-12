# Writing Code

> If you are using Rinf version 5 or earlier, please refer to the [historical docs](https://github.com/cunarist/rinf/blob/v5.4.0/documentation/docs/writing-code.md). With the introduction of Rinf version 6, a simpler way for communication between Dart and Rust has been implemented, and the system has undergone significant changes.

## 🏷️ Signal Details

### Code Generation

As you've seen in the tutorial, special comments inside `.proto` files allow Rinf's code generator invoked by `rinf message` to create appropriate channels for communication between Dart and Rust.

`[RINF:DART-SIGNAL]` generates a channel from Dart to Rust.

```proto
// Protobuf

// [RINF:DART-SIGNAL]
message MyDataInput { ... }
```

```dart
// Dart

myDataInputSend(MyDataInput( ... ), null);
```

```rust
// Rust

let receiver = my_data_input_receiver();
while let Some(my_data_input) = receiver.recv().await {
    // Custom Rust logic here
}
```

`[RINF:RUST-SIGNAL]` generates a channel from Rust to Dart.

```proto
// Protobuf

// [RINF:RUST-SIGNAL]
message MyDataOutput { ... }
```

```dart
// Dart

myDataOutputStream.listen((myDataOutput) {
    // Custom Dart logic here
})
```

```rust
// Rust

my_data_output_send(MyDataOutput{ ... }, None);
```

You can also provide binary data as the second argument of those `[]Send()` or `[]_send()` functions generated by Rinf. Its type should be `Uint8List?` in Dart and `Option<Vec<u8>>` in Rust. Passing binary data with this separate field is recommend over embedding it inside the Protobuf message, because it's more performant.

### Meanings of Each Field

We've covered how to pass signals between Dart and Rust in the previous tutorial section. Now, let's delve into the meaning of each field.

- **Field `message`:** It represents a message of a type defined by Protobuf. It's important to note that creating Protobuf messages larger than a few megabytes is not recommended. For large data, split them into multiple messages, or use `blob` instead. This field is mandatory.

- **Field `blob`:** This is a bytes array designed to handle large data, potentially up to a few gigabytes. You can send any kind of binary data you wish, such as a high-resolution image or file data. This field is optional and can be set to `null` or `None`.

Sending a serialized message or blob data is a zero-copy operation from Rust to Dart, while it involves a copy operation from Dart to Rust in memory. Keep in mind that the Protobuf serialization and deserialization process does involve memory copy.

### Efficiency

Rinf relies solely on native FFI for communication, avoiding the use of web protocols or hidden threads. The goal is to minimize performance overhead as much as possible.

## 📦 Message Code Generation

### Path

When you generate message code using the `rinf message` command, the resulting Dart and Rust modules' names and subpaths will precisely correspond to those of the `.proto` files.

- `./messages` : The `.proto` files under here and its subdirectories will be used.
- `./lib/messages` : The generated Dart code will be placed here.
- `./native/hub/src/messages` : The generated Rust code will be placed here.

### Continuous Watching

If you add the optional argument `-w` or `--watch` to the `rinf message` command, the message code will be automatically generated when `.proto` files are modified. If you add this argument, the command will not exit on its own.

```bash
rinf message --watch
```

## 🖨️ Printing for Debugging

You might be used to `println!` macro in Rust. However, using that macro isn't a very good idea in our apps made with Flutter and Rust because `println!` outputs cannot be seen on the web and mobile emulators.

When writing Rust code in the `hub` crate, you can simply print your debug message with the `debug_print!` macro provided by this framework like below. Once you use this macro, Flutter will take care of the rest.

```rust
crate::debug_print!("My object is {my_object:?}");
```

`debug_print!` is also better than `println!` because it only works in debug mode, resulting in a smaller and cleaner release binary.

## 🌅 Closing the App Gracefully

When the Flutter app is closed, the whole `tokio` runtime on the Rust side will be terminated automatically. However, some error messages can appear in the console if the Rust side sends messages to the Dart side even after the Dart VM has stopped. To prevent this, you can call `Rinf.finalize()` in Dart to terminate all Rust tasks before closing the Flutter app.

```dart
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';

...

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _appLifecycleListener = AppLifecycleListener(
    onExitRequested: () async {
      // Terminate Rust tasks before closing the Flutter app.
      await Rinf.finalize();
      return AppExitResponse.exit;
    },
  );

  @override
  void dispose() {
    _appLifecycleListener.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rinf Example',
      theme: ThemeData(
        useMaterial3: true,
        brightness: MediaQuery.platformBrightnessOf(context),
      ),
      home: MyHomePage(),
    );
  }
}

...
```
