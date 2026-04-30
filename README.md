built by using quartz by https://github.com/jackyzha0 and using their site as a template https://github.com/jackyzha0/jackyzha0.github.io

thank you jacky

quartz is [MIT](https://opensource.org/license/mit) licensed
jackyzha0.github.io is [MIT](https://opensource.org/license/mit) licensed
my code is [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) licensed
my writing is [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/deed.en) licensed

build system:

```sh
cargo run --locked --manifest-path tools/site/Cargo.toml -- build
cargo run --locked --manifest-path tools/site/Cargo.toml -- generate
cargo run --locked --manifest-path tools/site/Cargo.toml -- help
```
