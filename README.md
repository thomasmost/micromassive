[![Netlify Status](https://api.netlify.com/api/v1/badges/[YOUR_KEY_HERE]/deploy-status)](https://app.netlify.com/sites/yew-micromassive/deploys)

## About

An economy simulator built on Rust, WASM, and Yew

## 🚴 Development

### 🛠️ Build with `npm run build`

```
npm run build
```

### 🔬 Serve locally and watch for changes with `npm run start:dev`

```
npm run start:dev
```

## Current Notes
- Yew is running, but rendering a Frankenstein of todo components and Hunt-The-Wumpus controls as I learn the ins and outs
- scss files aren't watched yet, probably this can be handled in the webpack config

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
