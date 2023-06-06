#!/bin/bash

cargo build -p binder

forge clean

echo "Installing cozy contracts ... "
forge install https://github.com/Cozy-Finance/cozy-protocol-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-models-v2-simulation --no-commit
forge install https://github.com/Cozy-Finance/cozy-triggers-v2-simulation --no-commit

echo "Building contract artifacts ... "
for d in ./contracts/*/ ; do (cd "$d" && mv "test" "test_ignore"); done
for d in ./contracts/*/ ; do (cd "$d" && mv "script" "script_ignore"); done
for d in ./contracts/*/ ; do (cd "$d" && FOUNDRY_PROFILE='default' forge build); done

echo "Removing some artifacts ... "
rm -r "contracts/cozy-protocol-v2/out/structs"

echo "Building rust bindings ... "
./target/debug/binder --abis "contracts/cozy-triggers-v2-simulation/out" --bindings "lib/bindings" --mod-name "cozy_triggers" --overwrite
./target/debug/binder --abis "contracts/cozy-models-v2-simulation/out" --bindings "lib/bindings" --mod-name "cozy_models" --overwrite
./target/debug/binder --abis "contracts/cozy-protocol-v2/out" --bindings "lib/bindings" --mod-name "cozy_protocol" --overwrite