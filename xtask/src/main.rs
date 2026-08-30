use anyhow::{Context, Result};
use std::{env, fs, path::Path, process::Command};

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("ida-export") => ida_export(),
        Some("boost-audit") => boost_audit(),
        Some("verify") => verify(),
        Some("gen-skeleton") => {
            let krate = env::args().nth(2).unwrap_or_else(|| "reflection".into());
            println!("gen-skeleton for {krate}: placeholder — run `cargo xtask ida-export` first, then generate todo!() stubs per IDA ea");
            Ok(())
        }
        _ => {
            eprintln!("Usage: cargo xtask <ida-export|boost-audit|verify|gen-skeleton [crate]>");
            std::process::exit(1);
        }
    }
}

fn ida_export() -> Result<()> {
    // Calls IDA MCP POST /mcp via python one-liner to avoid adding reqwest dep
    let py = r#"
import urllib.request, json, pathlib
MCP="http://127.0.0.1:13337/mcp"
def call(tool, args):
    import urllib.request, json
    data=json.dumps({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool,"arguments":args}}).encode()
    req=urllib.request.Request(MCP, data=data, headers={"Content-Type":"application/json","Accept":"application/json"})
    with urllib.request.urlopen(req, timeout=120) as r:
        return json.loads(r.read())["result"]["structuredContent"]
def res(uri):
    import urllib.request, json
    data=json.dumps({"jsonrpc":"2.0","id":2,"method":"resources/read","params":{"uri":uri}}).encode()
    req=urllib.request.Request(MCP, data=data, headers={"Content-Type":"application/json","Accept":"application/json"})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read())
# export funcs
code=r"""
import idautils, ida_funcs, idc, json
funcs=list(idautils.Functions())
out=[{"ea": hex(ea), "mangled": ida_funcs.get_func_name(ea), "demangled": (idc.demangle_name(ida_funcs.get_func_name(ea),0) or ida_funcs.get_func_name(ea)), "type": (idc.get_type(ea) or "")} for ea in funcs]
open("/tmp/ida_export.json","w").write(json.dumps(out))
print(f"exported {len(out)} funcs to /tmp/ida_export.json")
"""
print(call("py_eval", {"code": code}))
# metadata
print(res("ida://idb/metadata"))
"#;
    let out = Command::new("python3").arg("-c").arg(py).output().context("python ida-export")?;
    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
    // Copy /tmp/ida_export.json if IDA wrote it (harness /tmp == IDA /tmp when same host)
    let tmp = Path::new("/tmp/ida_export.json");
    if tmp.exists() {
        fs::create_dir_all("ida")?;
        fs::copy(tmp, "ida/export.json")?;
        println!("copied /tmp/ida_export.json -> ida/export.json ({} bytes)", fs::metadata("ida/export.json")?.len());
        // also write metadata.json from MCP metadata resource (second call already printed; fetch again for file)
        // For now create minimal metadata.json if not exists
        if !Path::new("ida/metadata.json").exists() {
            let meta = serde_json::json!({
                "path": "/home/zzzzzzz/Downloads/ROBLOX 2.125.39335/Payload/robloxmobile.app/robloxmobile.i64",
                "module": "robloxmobile",
                "base": "0x4000",
                "size": "0x13a8efc",
                "md5": "b58125d0d78bcd63c97d7f2128563531",
                "sha256": "00e764c7c99995b05048ea8840d5d2155278ee571f668386d2b3d6a253d7bebe",
                "ipa": "/home/zzzzzzz/Downloads/ROBLOX 2.125.39335.ipa",
                "ipa_md5": "d4a442d7eb8d7e5063c6ca62f92e6667",
                "arch": "armv7",
                "funcs": 85545
            });
            fs::write("ida/metadata.json", serde_json::to_string_pretty(&meta)?)?;
        }
        let n: Vec<serde_json::Value> = serde_json::from_str(&fs::read_to_string("ida/export.json")?)?;
        println!("export.json: {} funcs", n.len());
        anyhow::ensure!(n.len() == 85545, "expected 85545 funcs, got {}", n.len());
    } else {
        anyhow::bail!("no /tmp/ida_export.json — is IDA running on 13337?");
    }
    Ok(())
}

fn boost_audit() -> Result<()> {
    let p = Path::new("ida/export.json");
    if !p.exists() { anyhow::bail!("run `cargo xtask ida-export` first — no ida/export.json"); }
    let data: Vec<serde_json::Value> = serde_json::from_str(&fs::read_to_string(p)?)?;
    let boost: Vec<_> = data.iter().filter(|v| {
        let m = v["mangled"].as_str().unwrap_or("");
        let d = v["demangled"].as_str().unwrap_or("");
        m.contains("boost") || d.contains("boost")
    }).collect();
    println!("total: {}, boost: {}", data.len(), boost.len());
    // per-crate estimate via name routing
    let mut counts = std::collections::BTreeMap::new();
    for v in &boost {
        let d = v["demangled"].as_str().unwrap_or("");
        let krate = if d.contains("Reflection") {"reflection"} else if d.contains("Instance")||d.contains("DataModel") {"datamodel"} else if d.contains("Ogre") {"rendering"} else if d.contains("RakNet") {"network"} else {"core"};
        *counts.entry(krate).or_insert(0) += 1;
    }
    for (k,v) in counts { println!("  {k}: {v}"); }
    Ok(())
}

fn verify() -> Result<()> {
    println!("cargo check --workspace");
    let s = Command::new("cargo").arg("check").arg("--workspace").status()?;
    anyhow::ensure!(s.success(), "cargo check failed");
    println!("cargo clippy --workspace -- -D warnings");
    let s = Command::new("cargo").args(["clippy","--workspace","--","-D","warnings"]).status()?;
    anyhow::ensure!(s.success(), "clippy failed");
    println!("verify: green");
    Ok(())
}
