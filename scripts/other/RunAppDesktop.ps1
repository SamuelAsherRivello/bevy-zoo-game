param(
    [switch]$CheckOnly,
    [switch]$Release,
    [switch]$UseSccache,
    [switch]$UseFastLinker,
    [switch]$NoFastLinker,
    [switch]$NoFastDevFeature,
    [string]$TargetTriple,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$PackageName = "bevy-zoo-game"
$TargetDir = Join-Path $RepositoryRoot "target\run-app-desktop"

if ($CheckOnly) {
    & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -AppOnly -Quiet
} else {
    & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet
}

$CompileParams = @{
    Action = if ($CheckOnly) { "check" } else { "build" }
    PackageName = $PackageName
    TargetDir = $TargetDir
    WgpuBackend = "dx12"
}
if ($TargetTriple) {
    $CompileParams.TargetTriple = $TargetTriple
}
if ($Release) {
    $CompileParams.Release = $true
} elseif (-not $NoFastDevFeature) {
    $CompileParams.Features = @("fast-dev")
}
if ($UseSccache) {
    $CompileParams.UseSccache = $true
}
if ($UseFastLinker) {
    $CompileParams.UseFastLinker = $true
}
if ($NoFastLinker) {
    $CompileParams.NoFastLinker = $true
}

Push-Location $RepositoryRoot
try {
    & (Join-Path $PSScriptRoot "..\other\CompileApp.ps1") @CompileParams @CargoArgs

    if (-not $CheckOnly) {
        $ProfileName = if ($Release) { "release" } else { "debug" }
        $TargetRoot = if ($TargetTriple) {
            Join-Path $TargetDir $TargetTriple
        } else {
            $TargetDir
        }
        $ExecutablePath = Join-Path $TargetRoot (Join-Path $ProfileName "$PackageName.exe")
        $ProfilePath = Join-Path $TargetRoot $ProfileName
        $DependencyPath = Join-Path $TargetRoot (Join-Path $ProfileName "deps")
        $RustSysroot = (rustc --print sysroot).Trim()
        $RustBinPath = Join-Path $RustSysroot "bin"

        if (-not (Test-Path $ExecutablePath)) {
            throw "Expected executable was not found: $ExecutablePath"
        }

        if (Test-Path $RustBinPath) {
            $env:PATH = "$RustBinPath;$env:PATH"
        }
        if (Test-Path $ProfilePath) {
            $env:PATH = "$ProfilePath;$env:PATH"
        }
        if (Test-Path $DependencyPath) {
            $env:PATH = "$DependencyPath;$env:PATH"
        }

        Write-Host "Opening desktop app: $ExecutablePath"
        Start-Process -FilePath $ExecutablePath -WorkingDirectory $RepositoryRoot
    }
} finally {
    Pop-Location
    if ($CheckOnly) {
        & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -AppOnly -Quiet
    }
}
