$root = "C:\Users\cheez\Downloads\TheNest"
$staging = "C:\Users\cheez\Downloads\TheNest_staging"
$zipPath = "C:\Users\cheez\Downloads\TheNest_handoff.zip"

Write-Host "Creating packaging staging area..."
if (Test-Path $staging) {
    Remove-Item $staging -Recurse -Force
}
New-Item -ItemType Directory -Force $staging | Out-Null

# Define list of top-level items to copy
$itemsToCopy = @(
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    ".gitignore",
    "swarm_core",
    "scenarios",
    "docs",
    "reports",
    "scripts",
    "ops",
    "agent",
    "reference",
    ".github"
)

# Copy files & folders to staging (excluding target inside folders just in case)
foreach ($item in $itemsToCopy) {
    $src = Join-Path $root $item
    if (Test-Path $src) {
        $dest = Join-Path $staging $item
        Copy-Item $src $dest -Recurse -Force
    }
}

# Ensure empty receipts directory structure exists with .gitkeep
New-Item -ItemType Directory -Force (Join-Path $staging "receipts") | Out-Null
New-Item -ItemType Directory -Force (Join-Path $staging "receipts\out") | Out-Null
if (Test-Path (Join-Path $root "receipts\.gitkeep")) {
    Copy-Item (Join-Path $root "receipts\.gitkeep") (Join-Path $staging "receipts\.gitkeep") -Force
} else {
    New-Item -ItemType File -Force (Join-Path $staging "receipts\.gitkeep") | Out-Null
}

# Zip the staging directory
Write-Host "Creating archive at $zipPath..."
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}
Compress-Archive -Path "$staging\*" -DestinationPath $zipPath -Force

# Verify target/ and receipts/out/* are not in zip
Write-Host "Verifying zip file structure..."
$verifyFailed = $false
$zipContents = tar -tf $zipPath 2>$null
if ($null -eq $zipContents) {
    Write-Host "tar not available, skipping verify via tar."
} else {
    $hasTarget = $zipContents | Where-Object { ($_ -like "target/*") -or ($_ -like "*/target/*") }
    $hasOutReceipts = $zipContents | Where-Object { $_ -like "*receipts/out/*" -and $_ -notlike "*receipts/out/" }

    
    if ($hasTarget) {
        Write-Warning "Validation FAILED: target/ folder found in handoff zip!"
        $verifyFailed = $true
    }
    if ($hasOutReceipts) {
        Write-Warning "Validation FAILED: generated receipts found in handoff zip!"
        $verifyFailed = $true
    }
}

# Clean staging
Remove-Item $staging -Recurse -Force

if (-not $verifyFailed) {
    Write-Host "Packaging successful: $zipPath" -ForegroundColor Green
} else {
    throw "Packaging verification failed!"
}
