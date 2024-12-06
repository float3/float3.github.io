import * as wasm from "wasm";

wasm
  .default()
  .then(() => {
    //make sure do anything that can call wasm after wasm has finished importing

  })
  .catch(console.error);