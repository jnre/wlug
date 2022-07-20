# wlug

window local users and group

hopefully it can run as a cli to manage the local users and group

TODO:

work on users first and hopefully on rust


## to build on macOS

```bash
brew install mingw-x64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

use `rustup target list` to see what target you have installed