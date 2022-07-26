# wlug

window local users and group

hopefully it can run as a cli to manage the local users and group


## to build on macOS

```bash
brew install mingw-x64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

use `rustup target list` to see what target you have installed

## to run complete

### have to create site-functions for zsh

```bash
cargo run -- --generate=zsh > /usr/local/share/zsh/site-functions/<name>
# cd to folder and chmod 755
# compinit
./target/debug/<name> --<TAB>
```

have yet to test for powershell and need to rewrite code for derive style 

### windows target

since it contains windows api, build this on windows computer

test:

```bash
cargo run --target=x86_64-pc-windows-gnu -- --generate=powershell > ./wlug
```
