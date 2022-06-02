# Build

```sh
wasm-pack build --target web
```

# Use

Import following generated files from JavaScript:

```
pkg/uugb_wasm_bg.wasm
pkg/uugb_wasm_bg.wasm.d.ts
pkg/uugb_wasm.d.ts
pkg/uugb_wasm.js
```

Call `init` and `start`.

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

Drop a ROM file into the canvas to start the game.
