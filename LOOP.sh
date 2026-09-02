#!/bin/bash
set -e
PROJECT="/home/zzzzzzz/rbx-2.125-rust"
cd "$PROJECT"
echo "[LOOP] $(date -u +%Y-%m-%dT%H:%M:%SZ) — omp loop started, PID $$"
echo "[LOOP] IDA MCP check..."
curl -s -X POST http://127.0.0.1:13337/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"resources/read","params":{"uri":"ida://idb/metadata"}}' | head -c 200; echo

# Loop until done — each iteration runs one crate via omp --print with muse-spark
# Global dedup: skip EAs already stubbed in ANY crate (fixes 3h stall at 42.5k)
CRATES=(core reflection datamodel rendering network audio script platform)
ITER=0
while true; do
  ITER=$((ITER+1))
  echo "================ LOOP ITER $ITER $(date -u +%Y-%m-%dT%H:%M:%SZ) ================"
  # Global unique count (accurate vs 85545)
  GLOBAL=$(grep -rh "pub fn stub_" crates --include="*.rs" 2>/dev/null | sed -n 's/.*stub_\(0x[0-9a-f]*\).*/\1/p' | sort -u | wc -l)
  echo "[LOOP] global unique: $GLOBAL/85545 ($((GLOBAL*100/85545))%)"
  if [ "$GLOBAL" -ge 85545 ]; then
    echo "[LOOP] SKELETON DONE — switching to impl phase (fill todo! bodies via decompile)"
  fi
  for CRATE in "${CRATES[@]}"; do
    echo "[LOOP] --- crate: $CRATE ---"
    COUNT=$(grep -r "0x" "crates/$CRATE/src" 2>/dev/null | wc -l || echo 0)
    echo "[LOOP] $CRATE current stubs: $COUNT (global unique $GLOBAL)"
    timeout 600 omp --model muse-spark --cwd "$PROJECT" --auto-approve --print "You are in $PROJECT. Read AGENT.md. Task: continue skeleton for $CRATE (rbx-$CRATE). Use ida/export.json (85545 funcs), filter for $CRATE namespace BUT FIRST run: grep -rh 'pub fn stub_' crates --include='*.rs' | sed -n 's/.*stub_\\(0x[0-9a-f]*\\).*/\\1/p' | sort -u > /tmp/global_eas.txt and SKIP any EA already in /tmp/global_eas.txt (global dedup). Only generate for EAs NOT in global set. Next 100 UNIQUE stubs with // 0xADDR — mangled + #[doc(alias)] + todo!(\"0xADDR\"), using rbx_core::SharedPtr not boost. After batch, cargo check --workspace must pass, then git add + commit. If all filtered EAs already in global set, reply GLOBAL DEDUP SKIP. Be brief." 2>&1 | tail -100
    echo "[LOOP] omp exit:$? for $CRATE"
    cargo check --workspace 2>&1 | tail -5
    git status --short 2>&1 | head -20
    if [ -n "$(git log origin/main..HEAD --oneline 2>/dev/null)" ]; then
      echo "[LOOP] pushing..."
      git push 2>&1 | tail -5
    fi
    echo "[LOOP] sleep 5"
    sleep 5
  done
  echo "[LOOP] completed full DAG pass $ITER, sleeping 30 before next"
  sleep 30
done
