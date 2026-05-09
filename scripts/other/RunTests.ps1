param(
    [switch]$NoFastDevFeature,
    [switch]$UseFastLinker,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$RunAppDesktopTargetDir = Join-Path $RepositoryRoot "target\run-app-desktop"

& (Join-Path $PSScriptRoot "StopApp.ps1") -Quiet

$CompileParams = @{
    Action = "test"
    Workspace = $true
    TargetDir = $RunAppDesktopTargetDir
}
if (-not $NoFastDevFeature) {
    $CompileParams.Features = @("fast-dev")
}
if ($UseFastLinker) {
    $CompileParams.UseFastLinker = $true
}

Push-Location $RepositoryRoot
try {
    & (Join-Path $PSScriptRoot "CompileApp.ps1") @CompileParams @CargoArgs
} finally {
    Pop-Location
}
