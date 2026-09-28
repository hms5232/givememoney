set default-list := true

[windows]
set shell := ["cmd.exe", "/c"]

alias wasm := serve-wasm

[group('wasm')]
build-wasm:
    cd wasm && wasm-pack build --target web
# build wasm, then serve a local server
[group('wasm')]
serve-wasm port="8000": build-wasm
    python -m http.server {{port}}
