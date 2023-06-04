#!/bin/bash

forge clean

echo "Installing cozy contracts ... "
forge install https://github.com/Cozy-Finance/cozy-protocol-v2 --no-commit
forge install https://github.com/Cozy-Finance/cozy-models-v2-simulation --no-commit
forge install https://github.com/Cozy-Finance/cozy-triggers-v2-simulation --no-commit

echo "Building contract artifacts ... "
for d in ./contracts/*/ ; do (cd "$d" && FOUNDRY_PROFILE=lite forge build --skip script test); done

echo "Building rust bindings ... "
cargo run -- --abis "contracts/cozy-triggers-v2-simulation/out" --bindings "lib/bindings" --mod-name "cozy_triggers" --overwrite
cargo run -- --abis "contracts/cozy-models-v2-simulation/out" --bindings "lib/bindings" --mod-name "cozy_models" --overwrite
cargo run -- --abis "contracts/cozy-protocol-v2/out" --bindings "lib/bindings" --mod-name "cozy_protocol" --overwrite