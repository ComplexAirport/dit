$ErrorActionPreference = "Stop"
$buildMode = "debug"
$outputDir = ".\"

if (-not (Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir | Out-Null
}

if ($buildMode -eq "release") {
    cargo build --release
    $targetPath = "target\release"
} else {
    cargo build
    $targetPath = "target\debug"
}

$execName = "dit_cli.exe"
$execDestName = "dit.exe"

$source = Join-Path $targetPath "$execName"
$destination = Join-Path $outputDir "$execDestName"

Copy-Item -Path $source -Destination $destination -Force

Write-Host "Built the executable to '$execDestName'"
