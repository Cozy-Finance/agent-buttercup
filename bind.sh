#!/bin/bash

cargo build -p binder

forge clean

echo "Installing cozy contracts ... "
forge install https://github.com/Cozy-Finance/cozy-protocol-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-models-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-triggers-v2 --no-commit

echo "Building contract artifacts ... "
for d in ./contracts/*/ ; do (cd "$d" && FOUNDRY_PROFILE=lite forge build --skip script test); done

echo "Building rust bindings ... "
./target/debug/binder --abis "contracts/cozy-triggers-v2/out" --bindings "lib/bindings" --mod-name "cozy_triggers" --overwrite
./target/debug/binder --abis "contracts/cozy-models-v2/out" --bindings "lib/bindings" --mod-name "cozy_models" --overwrite
./target/debug/binder --abis "contracts/cozy-protocol-v2/out" --bindings "lib/bindings" --mod-name "cozy_protocol" --overwrite