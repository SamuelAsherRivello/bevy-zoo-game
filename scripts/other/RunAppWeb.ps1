param(
    [switch]$CheckOnly,
    [switch]$Release,
    [switch]$NoOpen,
    [switch]$ExportOnly,
    [int]$Port = 8080,
    [string]$TargetTriple = "wasm32-unknown-unknown",
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$PackageName = "bevy-zoo-game"
$TargetDir = Join-Path $RepositoryRoot "target\run-app-web"
$WebRoot = Join-Path $TargetDir "site"
$SourceAssetsRoot = Join-Path (Join-Path (Join-Path (Join-Path $RepositoryRoot "bevy") "crates") "game") "assets"
$WebAssetsRoot = Join-Path $WebRoot "assets"
$ServerRoot = Join-Path $TargetDir "server"
$ServerScriptPath = Join-Path $ServerRoot "StaticFileServer.ps1"
$ServerPidPath = Join-Path $ServerRoot "server.pid"
$ServerLogPath = Join-Path $ServerRoot "server.log"
$ToolsRoot = Join-Path $TargetDir "tools"
$ToolsBin = Join-Path $ToolsRoot "bin"
$WasmBindgenVersion = "0.2.121"

function Test-CommandExists {
    param([Parameter(Mandatory = $true)][string]$Name)

    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Get-WasmBindgenCommand {
    $LocalWasmBindgen = Join-Path $ToolsBin "wasm-bindgen.exe"
    if (Test-Path $LocalWasmBindgen) {
        return $LocalWasmBindgen
    }

    $PathWasmBindgen = Get-Command "wasm-bindgen" -ErrorAction SilentlyContinue
    if ($PathWasmBindgen) {
        return $PathWasmBindgen.Source
    }

    return $null
}

function Install-LocalWasmBindgen {
    Write-Host "wasm-bindgen was not found. Installing wasm-bindgen-cli $WasmBindgenVersion into $ToolsRoot..."
    cargo install wasm-bindgen-cli --version $WasmBindgenVersion --root $ToolsRoot
    if ($LASTEXITCODE -ne 0) {
        throw "cargo install wasm-bindgen-cli failed with exit code $LASTEXITCODE."
    }

    $LocalWasmBindgen = Join-Path $ToolsBin "wasm-bindgen.exe"
    if (-not (Test-Path $LocalWasmBindgen)) {
        throw "Expected local wasm-bindgen was not found after install: $LocalWasmBindgen"
    }

    return $LocalWasmBindgen
}

function Stop-WebServer {
    if (-not (Test-Path $ServerPidPath)) {
        return
    }

    $ServerPidText = (Get-Content -Raw $ServerPidPath).Trim()
    if (-not $ServerPidText) {
        Remove-Item $ServerPidPath -Force
        return
    }

    $ServerProcess = Get-Process -Id ([int]$ServerPidText) -ErrorAction SilentlyContinue
    if ($ServerProcess) {
        Stop-Process -Id $ServerProcess.Id -Force
        Write-Host "Stopped previous web server ($($ServerProcess.Id))."
    }

    Remove-Item $ServerPidPath -Force -ErrorAction SilentlyContinue
}

function Write-StaticServerScript {
    New-Item -ItemType Directory -Force -Path $ServerRoot | Out-Null

    @'
param(
    [Parameter(Mandatory = $true)][string]$Root,
    [Parameter(Mandatory = $true)][int]$Port,
    [Parameter(Mandatory = $true)][string]$LogPath
)

$ErrorActionPreference = "Stop"

function Write-ServerLog {
    param([Parameter(Mandatory = $true)][string]$Message)

    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Add-Content -Path $LogPath -Value "[$Timestamp] $Message"
}

function Get-ContentType {
    param([Parameter(Mandatory = $true)][string]$Path)

    switch ([System.IO.Path]::GetExtension($Path).ToLowerInvariant()) {
        ".html" { return "text/html; charset=utf-8" }
        ".js" { return "text/javascript; charset=utf-8" }
        ".wasm" { return "application/wasm" }
        ".css" { return "text/css; charset=utf-8" }
        ".json" { return "application/json; charset=utf-8" }
        ".png" { return "image/png" }
        ".jpg" { return "image/jpeg" }
        ".jpeg" { return "image/jpeg" }
        ".svg" { return "image/svg+xml" }
        ".ico" { return "image/x-icon" }
        default { return "application/octet-stream" }
    }
}

$ResolvedRoot = (Resolve-Path $Root).Path
$Listener = [System.Net.HttpListener]::new()
$Listener.Prefixes.Add("http://127.0.0.1:$Port/")
$Listener.Start()
Write-ServerLog "Serving $ResolvedRoot at http://127.0.0.1:$Port/"

try {
    while ($Listener.IsListening) {
        $Context = $Listener.GetContext()
        try {
            $RequestPath = [System.Uri]::UnescapeDataString($Context.Request.Url.AbsolutePath.TrimStart("/"))
            if ([string]::IsNullOrWhiteSpace($RequestPath)) {
                $RequestPath = "index.html"
            }

            $CandidatePath = [System.IO.Path]::GetFullPath((Join-Path $ResolvedRoot $RequestPath))
            if (-not $CandidatePath.StartsWith($ResolvedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
                $Context.Response.StatusCode = 403
                $Context.Response.Close()
                continue
            }

            if (-not (Test-Path $CandidatePath -PathType Leaf)) {
                $Context.Response.StatusCode = 404
                $Context.Response.Close()
                continue
            }

            $Bytes = [System.IO.File]::ReadAllBytes($CandidatePath)
            $Context.Response.ContentType = Get-ContentType -Path $CandidatePath
            $Context.Response.ContentLength64 = $Bytes.Length
            $Context.Response.OutputStream.Write($Bytes, 0, $Bytes.Length)
            $Context.Response.Close()
        } catch {
            Write-ServerLog "Request failed: $($_.Exception.Message)"
            try {
                $Context.Response.Close()
            } catch {
            }
        }
    }
} catch {
    Write-ServerLog $_.Exception.Message
    throw
} finally {
    if ($Listener.IsListening) {
        $Listener.Stop()
    }
    $Listener.Close()
}
'@ | Set-Content -Path $ServerScriptPath -Encoding UTF8
}

if (-not $ExportOnly) {
    & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet
    Stop-WebServer
}

if ($TargetTriple -ne "wasm32-unknown-unknown") {
    Write-Warning "RunAppWeb is intended for wasm32-unknown-unknown. Current target: $TargetTriple"
}

$InstalledTargets = rustup target list --installed
if ($InstalledTargets -notcontains $TargetTriple) {
    throw "Rust target '$TargetTriple' is not installed. Run 'rustup target add $TargetTriple' and rerun this script."
}

if ($CheckOnly) {
    $CompileAction = "check"
} else {
    $WasmBindgenCommand = Get-WasmBindgenCommand
    if (-not $WasmBindgenCommand) {
        $WasmBindgenCommand = Install-LocalWasmBindgen
    }
    $CompileAction = "build"
}

$CompileParams = @{
    Action = $CompileAction
    PackageName = $PackageName
    TargetDir = $TargetDir
    TargetTriple = $TargetTriple
    NoFastLinker = $true
}
if ($Release) {
    $CompileParams.Release = $true
}

if ($CheckOnly) {
    Write-Host "Mode: check only"
} elseif ($ExportOnly) {
    Write-Host "Mode: build web bundle"
} elseif ($NoOpen) {
    Write-Host "Mode: build web bundle and serve"
} else {
    Write-Host "Mode: build web bundle and open browser"
}

Push-Location $RepositoryRoot
try {
    & (Join-Path $PSScriptRoot "..\other\CompileApp.ps1") @CompileParams @CargoArgs

    if (-not $CheckOnly) {
        $ProfileName = if ($Release) { "release" } else { "debug" }
        $WasmFileNames = @(
            "$PackageName.wasm",
            "$($PackageName.Replace('-', '_')).wasm"
        )
        $WasmPath = $null
        foreach ($WasmFileName in $WasmFileNames) {
            $CandidatePath = Join-Path $TargetDir (Join-Path $TargetTriple (Join-Path $ProfileName $WasmFileName))
            if (Test-Path $CandidatePath) {
                $WasmPath = $CandidatePath
                break
            }
        }

        if (-not $WasmPath) {
            throw "Expected WebAssembly artifact was not found under $(Join-Path $TargetDir (Join-Path $TargetTriple $ProfileName))."
        }

        New-Item -ItemType Directory -Force -Path $WebRoot | Out-Null
        if (Test-Path $SourceAssetsRoot) {
            New-Item -ItemType Directory -Force -Path $WebAssetsRoot | Out-Null
            Copy-Item -Path (Join-Path $SourceAssetsRoot "*") -Destination $WebAssetsRoot -Recurse -Force
        }

        $BindgenCommand = @(
            "--target", "web",
            "--out-dir", $WebRoot,
            "--out-name", "bevy_zoo_game",
            $WasmPath
        )
        Write-Host "wasm-bindgen command: $WasmBindgenCommand $($BindgenCommand -join ' ')"
        & $WasmBindgenCommand @BindgenCommand
        if ($LASTEXITCODE -ne 0) {
            throw "wasm-bindgen failed with exit code $LASTEXITCODE."
        }

        @'
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Bevy Zoo Game</title>
    <style>
        html,
        body {
            width: 100%;
            height: 100%;
            margin: 0;
            background: #111;
            overflow: hidden;
        }

        canvas {
            display: block;
            width: 100vw !important;
            height: 100vh !important;
        }
    </style>
</head>
<body>
    <script type="module">
        import init from "./bevy_zoo_game.js";

        init().catch((error) => {
            console.error(error);
            document.body.textContent = `Failed to start Bevy Zoo Game: ${error}`;
        });
    </script>
</body>
</html>
'@ | Set-Content -Path (Join-Path $WebRoot "index.html") -Encoding UTF8

        if ($ExportOnly) {
            Write-Host "Exported web app: $WebRoot"
            return
        }

        Write-StaticServerScript
        if (Test-Path $ServerLogPath) {
            Remove-Item $ServerLogPath -Force
        }

        $PowerShellExe = (Get-Process -Id $PID).Path
        $ServerArgs = @(
            "-NoLogo",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-File", $ServerScriptPath,
            "-Root", $WebRoot,
            "-Port", $Port,
            "-LogPath", $ServerLogPath
        )
        $ServerProcess = Start-Process -FilePath $PowerShellExe -ArgumentList $ServerArgs -WindowStyle Hidden -PassThru
        Set-Content -Path $ServerPidPath -Value $ServerProcess.Id -Encoding ASCII

        $Url = "http://127.0.0.1:$Port/"
        Start-Sleep -Milliseconds 500
        Write-Host "Serving web app: $Url"
        if (-not $NoOpen) {
            Write-Host "Opening browser: $Url"
            Start-Process $Url
        }
    }
} finally {
    Pop-Location
}
