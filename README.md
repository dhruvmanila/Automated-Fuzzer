## Automated Fuzzer

This repo contains simple tool to create broken files and checking them with special apps(rome, ruff, mypy and many more already are implemented, but it is easy to add support for any different app).

This small tool I created mainly for my own use without much vision, so you can easily compile app without changing source code
if you want to use already implemented fuzzers, but if you want to test your own app

This tool is designed for fast iterations, so it works really great if your app can test/lint several files per second(e.g. ruff on my pc can test even 50 middle size files per second in one core).

## How to use it?
- Install tool to create broken files
```
cargo install create_broken_files
```
- Create file inside `apps` folder and customize class name
- Customize run command, broken messages or created files
- Add to MODE enum your app and point at new file in `main.rs` in match statement
- Create setting inside `fuzz_settings.toml`
- Create required folders used inside `fuzz_settings.toml`
- Find "valid" files - you can find a lot of files in github by cloning big repos and checking its files - https://github.com/search?q=stars%3A%3E50++language%3ARust+size%3A%3E1000&type=repositories
- Run app via `cargo run --release`

## How this works
- At start app take n valid files from folder
- Depending on settings invalid files are created
- In loop, different app(`ruff`, `rome`, `mypy` etc.) check this file
- Basing on output messages like `RUST_BACKTRACE`, `crashed`, `error`, `internal bug`, file is checked if caused some bugs(this allow to find not only crashes).
- If it found it, then this file is copied to special folder
- If minimization is enabled, app tries to minimize output to produce bug(this may take some time, but output files are usually smaller 2x-100x times)

Video, how output should look:  

https://user-images.githubusercontent.com/41945903/227783281-a73112ee-b564-41f3-9d6a-f63b294abbce.mp4

## It really works?
Yes, it found thousands of crashes in several projects(most are implemented as examples):
- Selene - https://github.com/Kampfkarren/selene/issues/505 (1375 files)
- Rome - https://github.com/rome/tools/issues/4323 (>2000 files)
- Ruff - https://github.com/charliermarsh/ruff/issues/3721 (>2000 files)
- Symphonia - https://github.com/pdeljanov/Symphonia/issues/201, https://github.com/pdeljanov/Symphonia/issues/200 (30 files)
- Lofty - https://github.com/Serial-ATA/lofty-rs/issues/174 - (1 file)
- Deno lint - https://github.com/denoland/deno_lint/issues/1145 - (873 files)
- Oxc - https://github.com/Boshen/oxc/issues/232 - (>300 files)