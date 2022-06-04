## Build

```sh
wasm-pack build --release --target web
```

## Use

Copy following generated files:

```
pkg/uugb_wasm_bg.wasm
pkg/uugb_wasm_bg.wasm.d.ts
pkg/uugb_wasm.d.ts
pkg/uugb_wasm.js
```

Import `uugb_wasm.js`.

Call `init` and `start`.

Drop a ROM file into the canvas to start the game.

## Example

```js
<canvas id="canvas"></canvas>
<script type="module">
    import init, {start} from "./uugb_wasm.js";
    (async () => {
        await init();
        start('canvas');
    })();
</script>
```
