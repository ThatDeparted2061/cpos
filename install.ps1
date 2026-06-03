param(
  [string]$Version = $env:CPOS_VERSION,
  [string]$InstallDir = $env:CPOS_INSTALL_DIR,
  [string]$Repo = $env:CPOS_REPO
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($Version)) {
  $Version = "latest"
}

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
  $InstallDir = Join-Path $env:LOCALAPPDATA "Programs\CPOS\bin"
}

if ([string]::IsNullOrWhiteSpace($Repo)) {
  $Repo = "Soham109/cpos"
}

if (-not [Environment]::Is64BitOperatingSystem) {
  throw "CPOS currently ships a Windows x64 TUI binary. Use a 64-bit Windows machine or build from source."
}

$target = "x86_64-pc-windows-msvc"
$asset = "cpos-$target.zip"

if ($Version -eq "latest") {
  $url = "https://github.com/$Repo/releases/latest/download/$asset"
} else {
  if ($Version.StartsWith("v")) {
    $tag = $Version
  } else {
    $tag = "v$Version"
  }
  $url = "https://github.com/$Repo/releases/download/$tag/$asset"
}

$tmp = Join-Path ([IO.Path]::GetTempPath()) ("cpos-install-" + [guid]::NewGuid().ToString("N"))
$archive = Join-Path $tmp $asset

try {
  New-Item -ItemType Directory -Force -Path $tmp | Out-Null
  New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

  Write-Host "Installing CPOS TUI for $target"
  Write-Host "Downloading $url"
  Invoke-WebRequest -Uri $url -OutFile $archive -UseBasicParsing

  Expand-Archive -Path $archive -DestinationPath $tmp -Force

  $binary = Join-Path $tmp "cpos.exe"
  if (-not (Test-Path $binary)) {
    throw "Release archive did not contain cpos.exe"
  }

  $dest = Join-Path $InstallDir "cpos.exe"
  Copy-Item $binary $dest -Force

  $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $pathParts = @()
  if (-not [string]::IsNullOrWhiteSpace($userPath)) {
    $pathParts = $userPath.Split(";") | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
  }

  $alreadyOnPath = $false
  foreach ($part in $pathParts) {
    if ($part.TrimEnd("\") -ieq $InstallDir.TrimEnd("\")) {
      $alreadyOnPath = $true
      break
    }
  }

  if (-not $alreadyOnPath) {
    $newPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $InstallDir } else { "$userPath;$InstallDir" }
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = "$env:Path;$InstallDir"
    Write-Host "Added $InstallDir to your user PATH. Restart your terminal if 'cpos' is not found."
  }

  Write-Host "Installed CPOS to $dest"
  Write-Host "Run: cpos"
} finally {
  Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
