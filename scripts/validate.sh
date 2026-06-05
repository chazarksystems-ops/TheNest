#!/bin/bash
set -e

RUN_BENCH=false
RUN_DEMO=false
RUN_STRESS=false

for arg in "$@"
do
    case $arg in
        -Bench|--bench)
        RUN_BENCH=true
        shift
        ;;
        -Demo|--demo)
        RUN_DEMO=true
        shift
        ;;
        -Stress|--stress)
        RUN_STRESS=true
        shift
        ;;
    esac
done

echo "Running validation suite..."
cargo fmt --all --check
cargo check --workspace
cargo test --workspace

if [ "$RUN_BENCH" = true ] ; then
    echo "Running benchmarks..."
    cargo bench
fi

if [ "$RUN_DEMO" = true ] ; then
    echo "Running scenario runner demo..."
    cargo run --bin demo -- healthy
    cargo run --bin demo -- breach
    cargo run --bin demo -- exact
    cargo run --bin demo -- below
fi

if [ "$RUN_STRESS" = true ] ; then
    echo "Running batch stress harness..."
    cargo run --release --bin batch_stress
fi

echo "Validation passed!"

