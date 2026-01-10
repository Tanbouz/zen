[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# Web/WASM Rules Engine

ZEN Engine is a cross-platform, Open-Source Business Rules Engine (BRE). It is written in **Rust** and provides native bindings for **NodeJS**, **Python**, **Go**, and **Web/WASM**. ZEN Engine allows to load and execute [JSON Decision Model (JDM)](https://gorules.io/docs/rules-engine/json-decision-model) from JSON files.

<img width="800" alt="Open-Source Rules Engine" src="https://gorules.io/images/jdm-editor.gif">

An open-source React editor is available on our [JDM Editor](https://github.com/gorules/jdm-editor) repo.

## Usage

The Web binding provides ZEN Engine for browser and serverless environments (Deno, Cloudflare Workers, etc.).

### Installation

#### Using wasm-pack (Build from source)

```bash
cd bindings/web
wasm-pack build --target web --out-dir ../../pkg
```

This will generate WASM bindings in the `pkg/` directory at the repository root.

### Simple Example

```typescript
import init, { ZenEngine } from './zen_web.js';

// Initialize WASM module
const wasmBytes = await Deno.readFile(new URL('./zen_web_bg.wasm', import.meta.url));
await init(wasmBytes);

// Load decision model
const decisionModel = JSON.parse(
  await Deno.readTextFile('./my-decision.json')
);

// Create engine instance
const engine = new ZenEngine(decisionModel);

// Evaluate with input context
const result = await engine.evaluate({ 
  customerAge: 25,
  purchaseAmount: 150 
});

console.log(result);

// Free WASM memory
engine.free();
```

### Example with Deno / Supabase Edge Functions

```typescript
import init, { ZenEngine } from './zen_web.js';

// Initialize once at module load
const wasmBytes = await Deno.readFile(new URL('./zen_web_bg.wasm', import.meta.url));
await init(wasmBytes);

const decisionModel = JSON.parse(
  await Deno.readTextFile(new URL('./decision.json', import.meta.url))
);

Deno.serve(async (req) => {
  const input = await req.json();
  
  const engine = new ZenEngine(decisionModel);
  const result = await engine.evaluate(input);
  engine.free();
  
  return Response.json(result);
});
```

### Platform Limitations

**QuickJS Function Nodes Not Supported:**  
The Web binding is compiled **without QuickJS support** because QuickJS requires native threading and platform-specific features that aren't available in WASM environments.

This means:
- ✅ Decision Tables work
- ✅ Expression Nodes work
- ✅ Switch Nodes work
- ❌ Function Nodes (v1 and v2) will return an error

If your decision models use Function Nodes, consider:
1. Using the NodeJS/Python/Rust bindings instead
2. Converting Function Nodes to Expression Nodes where possible
3. Running function logic in your application code before/after evaluation

### Supported Platforms

List of platforms where Zen Engine is natively available:

* **NodeJS** - [GitHub](https://github.com/gorules/zen/blob/master/bindings/nodejs/README.md) | [Documentation](https://gorules.io/docs/developers/bre/engines/nodejs) | [npmjs](https://www.npmjs.com/package/@gorules/zen-engine)
* **Python** - [GitHub](https://github.com/gorules/zen/blob/master/bindings/python/README.md) | [Documentation](https://gorules.io/docs/developers/bre/engines/python) | [pypi](https://pypi.org/project/zen-engine/)
* **Go** - [GitHub](https://github.com/gorules/zen-go) | [Documentation](https://gorules.io/docs/developers/bre/engines/go)
* **Rust (Core)** - [GitHub](https://github.com/gorules/zen) | [Documentation](https://gorules.io/docs/developers/bre/engines/rust) | [crates.io](https://crates.io/crates/zen-engine)
* **Web/WASM** - [GitHub](https://github.com/gorules/zen/blob/master/bindings/web/README.md)

For a complete **Business Rules Management Systems (BRMS)** solution:

* [Self-hosted BRMS](https://gorules.io)
* [GoRules Cloud BRMS](https://gorules.io/signin/verify-email)

## Building

### Requirements

- Rust 1.70+
- wasm-pack (`cargo install wasm-pack`)

### Build Command

```bash
# For web targets (default)
wasm-pack build --target web

# For Node.js
wasm-pack build --target nodejs

# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler
```

The output will be in `pkg/` directory containing:
- `zen_web_bg.wasm` - The compiled WASM binary
- `zen_web.js` - JavaScript glue code
- `zen_web.d.ts` - TypeScript definitions

## Performance

WASM bindings provide near-native performance while maintaining portability across different JavaScript runtimes. For CPU-intensive decision models with many rules, WASM typically outperforms pure JavaScript implementations by 2-10x.

## License

MIT - See [LICENSE](../../LICENSE) file for details.

## Contributing

Contributions are welcome! Please see the main [repository](https://github.com/gorules/zen) for contribution guidelines.
