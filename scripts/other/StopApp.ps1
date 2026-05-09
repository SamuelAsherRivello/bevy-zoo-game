param(
    [switch]$AppOnly,
    [switch]$Quiet
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$PackageName = "bevy-zoo-game"
$WebServerPidPath = Join-Path $RepositoryRoot "target\run-app-web\server\server.pid"
$AppProcessNames = @(
    "bevy-zoo-game.exe",
    "bevy_zoo_game.exe"
)
$ProjectPathPattern = [regex]::Escape($RepositoryRoot)
$StoppedCount = 0

$Processes = Get-CimInstance Win32_Process |
    Where-Object {
        $_.Name -in $AppProcessNames -or
        (
            -not $AppOnly -and
            $_.CommandLine -and
            $_.CommandLine -match $ProjectPathPattern -and
            ($_.Name -in @("cargo.exe", "dx.exe", "rustc.exe", "rust-lld.exe", "link.exe") -or $_.Name -in $AppProcessNames)
        )
    }

foreach ($Process in $Processes) {
    if ($Process.ProcessId -eq $PID) {
        continue
    }

    try {
        Stop-Process -Id $Process.ProcessId -Force -ErrorAction Stop
        $StoppedCount += 1
        if (-not $Quiet) {
            Write-Host "Stopped $($Process.Name) ($($Process.ProcessId))."
        }
    } catch {
        if (-not $Quiet) {
            Write-Warning "Could not stop $($Process.Name) ($($Process.ProcessId)): $($_.Exception.Message)"
        }
    }
}

if ((-not $AppOnly) -and (Test-Path $WebServerPidPath)) {
    $WebServerPidText = (Get-Content -Raw $WebServerPidPath).Trim()
    if ($WebServerPidText) {
        $WebServerProcess = Get-Process -Id ([int]$WebServerPidText) -ErrorAction SilentlyContinue
        if ($WebServerProcess) {
            try {
                Stop-Process -Id $WebServerProcess.Id -Force -ErrorAction Stop
                $StoppedCount += 1
                if (-not $Quiet) {
                    Write-Host "Stopped web server ($($WebServerProcess.Id))."
                }
            } catch {
                if (-not $Quiet) {
                    Write-Warning "Could not stop web server ($($WebServerProcess.Id)): $($_.Exception.Message)"
                }
            }
        }
    }

    Remove-Item $WebServerPidPath -Force -ErrorAction SilentlyContinue
}

if (-not $Quiet) {
    if ($StoppedCount -eq 0) {
        Write-Host "No running project processes found."
    } else {
        Write-Host "Stopped $StoppedCount project process(es)."
    }
}
