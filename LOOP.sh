#!/bin/bash
set -e
PROJECT="/home/zzzzzzz/rbx-2.125-rust"
cd "$PROJECT"
echo "[LOOP] $(date -u +%Y-%m-%dT%H:%M:%SZ) — omp loop started, PID $$"
echo "[LOOP] IDA MCP check..."
curl -s -X POST http://127.0.0.1:13337/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"resources/read","params":{"uri":"ida://idb/metadata"}}' | head -c 200; echo

# Loop until done — each iteration runs one crate via omp --print with auto-approve
# Crate order follows DAG: core -> reflection -> datamodel -> rendering -> network -> audio -> script -> platform
CRATES=(core reflection datamodel rendering network audio script platform)
ITER=0
while true; do
  ITER=$((ITER+1))
  echo "================ LOOP ITER $ITER $(date -u +%Y-%m-%dT%H:%M:%SZ) ================"
  for CRATE in "${CRATES[@]}"; do
    echo "[LOOP] --- crate: $CRATE ---"
    # Check if crate already has significant skeletons (rg count)
    COUNT=$(grep -r "0x" "crates/$CRATE/src" 2>/dev/null | wc -l || echo 0)
    echo "[LOOP] $CRATE current stubs: $COUNT"
    # Run omp for this crate — generates next 100 funcs, verifies, commits
    timeout 600 omp --model muse-spark --cwd "$PROJECT" --auto-approve --print "You are in $PROJECT. Read AGENT.md. Task: continue skeleton generation for crate $CRATE (rbx-$CRATE). Use ida/export.json (85545 funcs), filter demangled for that crate's namespace (Reflection->reflection, Instance/DataModel->datamodel, Ogre->rendering, RakNet->network, etc). Generate next 100 stubs with // 0xADDR — mangled + #[doc(alias)] + todo!(\"0xADDR\"), using rbx_core::SharedPtr not boost. After batch, cargo check --workspace must pass, then git add + commit. If crate is done (all funcs stubbed), skip. Be brief." 2>&1 | tail -100
    echo "[LOOP] omp exit:$? for $CRATE"
    cargo check --workspace 2>&1 | tail -5
    git status --short 2>&1 | head -20
    # Push if commits made
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
