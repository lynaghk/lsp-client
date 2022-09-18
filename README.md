How long does it take rust-analyzer to autocomplete from 32k items?

It's 10s from Emacs with lsp-mode ([details](https://gist.github.com/lynaghk/e5cca329418e32b0644ef209d8194895)) so I threw this code together to test rust-analyzer itself.

On my M1 MacBook Air with 16 GB of RAM,

    rust-analyzer 0.3.1203-standalone (2e9f1204c 2022-09-11)

takes about 3s.

Run it yourself via:

    cargo run --release

See also the [discussion thread on Rust User Forum](https://users.rust-lang.org/t/how-many-symbols-can-rust-analyzer-autocomplete-in-practice/81400).
