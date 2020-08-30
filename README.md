# reMarkable Input Grab

This is a simple program to grab events from a input device (`/dev/input/eventX`) so no other program can receive the event.

# Usage

```bash
# grab input events from /dev/input/event1 for 3 seconds
./rm_grab /dev/input/event1 3
```

For reMarkable 1, `/dev/input/event0` is pen input, `/dev/input/event1` is touchscreen and `/dev/input/event2` is buttons. (According to [libremarkable/src/input/ev.rs#L57](https://github.com/canselcik/libremarkable/blob/f62cedf7d2bb27b9abf53eda589f8c92c851d26d/src/input/ev.rs#L57))

# Build

1. Install [Cross](https://github.com/rust-embedded/cross).
2. Build a statically linked binary with musl.
   ```bash
    cross build --target armv7-unknown-linux-musleabihf --release
   ```

Precompiled binary is available in the release page
