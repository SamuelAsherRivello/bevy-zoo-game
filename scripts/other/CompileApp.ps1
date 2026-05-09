param(
    [ValidateSet("build", "check", "test")]
    [string]$Action = "build",
    [string]$PackageName,
    [switch]$Workspace,
    [switch]$Release,
    [string]$TargetTriple,
    [Parameter(Mandatory = $true)]
    [string]$TargetDir,
    [string[]]$Features,
    [switch]$UseSccache,
    [switch]$UseFastLinker,
    [switch]$NoFastLinker,
    [string]$WgpuBackend,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$ResolvedTargetDir = if ([System.IO.Path]::IsPathRooted($TargetDir)) {
    $TargetDir
} else {
    Join-Path $RepositoryRoot $TargetDir
}

if ($PackageName -and $Workspace) {
    throw "Use either -PackageName or -Workspace, not both."
}

if ((-not $PackageName) -and (-not $Workspace)) {
    throw "CompileApp requires -PackageName or -Workspace."
}

$env:CARGO_TARGET_DIR = $ResolvedTargetDir

if ($WgpuBackend) {
    $env:WGPU_BACKEND = $WgpuBackend
}

if ($UseSccache) {
    $Sccache = Get-Command "sccache" -ErrorAction SilentlyContinue
    if ($Sccache) {
        $env:CARGO_INCREMENTAL = "0"
        $env:RUSTC_WRAPPER = $Sccache.Source
        $env:SCCACHE_DIR = Join-Path $env:CARGO_TARGET_DIR "sccache"
        Write-Host "Using sccache: $($Sccache.Source)"
    } else {
        throw "sccache was requested but was not found on PATH."
    }
} else {
    $env:CARGO_INCREMENTAL = "1"
    if ($env:RUSTC_WRAPPER -like "*sccache*") {
        Remove-Item Env:\RUSTC_WRAPPER
    }
    Write-Host "Using Cargo incremental compilation. Pass -UseSccache to use sccache instead."
}

if (Test-Path Env:\CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER) {
    Remove-Item Env:\CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER
}

if ($UseFastLinker -and -not $NoFastLinker) {
    $LldLink = Get-Command "lld-link" -ErrorAction SilentlyContinue
    if ($LldLink) {
        $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = $LldLink.Source
        Write-Host "Using fast linker: $($LldLink.Source)"
    } else {
        throw "lld-link was requested but was not found on PATH."
    }
} elseif (-not $NoFastLinker) {
    Write-Host "Using the default Windows linker. Pass -UseFastLinker to use lld-link."
}

$CargoCommand = @($Action)
if ($Workspace) {
    $CargoCommand += "--workspace"
} else {
    $CargoCommand += @("-p", $PackageName)
}

if ($TargetTriple) {
    $CargoCommand += @("--target", $TargetTriple)
}

if ($Release) {
    $CargoCommand += "--release"
}

if ($Features) {
    $CargoCommand += @("--features", ($Features -join ","))
}

if ($CargoArgs) {
    $CargoCommand += $CargoArgs
}

if ($TargetTriple) {
    Write-Host "Target: $TargetTriple"
} else {
    Write-Host "Target: host default"
}
Write-Host "Mode: cargo $Action"
if ($WgpuBackend) {
    Write-Host "WGPU backend: $env:WGPU_BACKEND"
}
Write-Host "Incremental builds: $env:CARGO_INCREMENTAL"
if ($Features) {
    Write-Host "Features: $($Features -join ',')"
}
Write-Host "Cargo target dir: $env:CARGO_TARGET_DIR"
Write-Host "Cargo command: cargo $($CargoCommand -join ' ')"

Push-Location $RepositoryRoot
try {
    cargo @CargoCommand
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo command failed with exit code $LASTEXITCODE."
    }
} finally {
    Pop-Location
}
