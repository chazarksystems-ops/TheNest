param(
    [switch]$Bench,
    [switch]$Demo,
    [switch]$Stress
)

$ErrorActionPreference = "Stop"
Write-Host "Running validation suite..."
cargo fmt --all --check
cargo check --workspace
cargo test --workspace

if ($Bench) {
    Write-Host "Running benchmarks..." -ForegroundColor Cyan
    cargo bench
}

if ($Demo) {
    Write-Host "Running scenario runner demo..." -ForegroundColor Cyan
    cargo run --bin demo -- healthy
    cargo run --bin demo -- breach
    cargo run --bin demo -- exact
    cargo run --bin demo -- below
}

if ($Stress) {
    Write-Host "Running batch stress harness..." -ForegroundColor Cyan
    cargo run --release --bin batch_stress
}

Write-Host "Validation passed!" -ForegroundColor Green

