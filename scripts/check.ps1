Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Set-Variable -Name PSNativeCommandUseErrorActionPreference -Value $true -Scope Global -ErrorAction SilentlyContinue

# Determine paths
$ScriptDir = $PSScriptRoot
$RepoRoot = (Resolve-Path (Join-Path $ScriptDir '..')).Path

$checkPaths = @(
    (Join-Path $RepoRoot 'qemu-plugin'),
    (Join-Path $RepoRoot 'qemu-plugin-sys'),
    (Join-Path $RepoRoot 'plugins/icount'),
    (Join-Path $RepoRoot 'plugins/tiny'),
    (Join-Path $RepoRoot 'plugins/tiny-system'),
    (Join-Path $RepoRoot 'plugins/tracer')
)

# Run cargo fmt from repo root to ensure workspace is detected
Push-Location $RepoRoot
try {
    & cargo fmt --all --check
} finally {
    Pop-Location
}

$commonArgs = @(
    '--mutually-exclusive-features=plugin-api-v0,plugin-api-v1,plugin-api-v2,plugin-api-v3,plugin-api-v4,plugin-api-v5',
    '--at-least-one-of=plugin-api-v0,plugin-api-v1,plugin-api-v2,plugin-api-v3,plugin-api-v4,plugin-api-v5',
    '--feature-powerset',
    '--exclude-no-default-features'
)

foreach ($checkPath in $checkPaths) {
    $manifestPath = Join-Path $checkPath 'Cargo.toml'

    & cargo +nightly hack --manifest-path $manifestPath @commonArgs check
    & cargo +nightly hack --manifest-path $manifestPath @commonArgs clippy
}
