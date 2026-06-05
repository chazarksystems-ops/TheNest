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
    Write-Host "Running scenario runner demo (hive_workbench)..." -ForegroundColor Cyan
    cargo run --bin hive_workbench -- run healthy
    cargo run --bin hive_workbench -- run breach
    cargo run --bin hive_workbench -- run exact
    cargo run --bin hive_workbench -- run below
}

if ($Stress) {
    Write-Host "Running batch stress harness..." -ForegroundColor Cyan
    cargo run --release --bin batch_stress
}

Write-Host "Validation passed!" -ForegroundColor Green
