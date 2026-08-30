# ida

Ground truth is IDA DB at `/home/zzzzzzz/Downloads/ROBLOX 2.125.39335/Payload/robloxmobile.app/robloxmobile.i64` (MCP 127.0.0.1:13337).

Exports (not DB):

- `export.json` — ~85,545 funcs `{ea, mangled, demangled, type}` — `cargo xtask ida-export`
- `metadata.json` — arch/base/hashes

Regenerate: `cargo xtask ida-export && cargo xtask boost-audit`
