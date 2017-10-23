# Conrod Soft KeyPad
This is a basic sample crate for creating a soft KeyPad for Mobile with Rust using Conrod with several backend options. Conrod allows different graphical backend like "glium", "winit" or even "SDL2". 

# Running examples on deskop

```
cargo run --example keypad
```

# Running examples on Android
For Android devices, this is using Tomaka's android-glue to compile into apk. For more information of how to setup the android-glue, please visit https://github.com/tomaka/android-rs-glue

After setting up the android-glue, follow the command to build the apk.
```
cargo apk build --example keypad
adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk
```

# Running examples on IOS
WIP