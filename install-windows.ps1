<#
.SYNOPSIS
Installs or updates the Martensite development dependencies on Windows.

.DESCRIPTION
Uses winget for everything it can. The script is safe to run again: missing
packages are installed and installed packages are upgraded.

What it sets up:
  - Visual Studio Build Tools with the C++ workload (MSVC), Git, GitHub CLI,
    CMake and pkg-config-lite
  - Vulkan SDK (loader, validation layers, vulkaninfo, glslang, SPIRV-Tools)
  - rustup, the toolchain pinned in rust-toolchain.toml, and the cargo tools
    the gates use (cargo-deny, cargo-audit, cargo-nextest, cargo-llvm-cov)
  - Tcl/Tk 8.6 and FreeType, built from pinned sources with MSVC, because no
    Windows package ships them with the MSVC ABI that Rust links against

Built libraries go to %LOCALAPPDATA%\Martensite\deps. The script sets
PKG_CONFIG_PATH, TCL_LIBRARY, TK_LIBRARY and PATH for the current user.

.PARAMETER NoCargoTools
Skip installing and updating the cargo tools.

.EXAMPLE
powershell -ExecutionPolicy Bypass -File .\install-windows.ps1
#>
[CmdletBinding()]
param([switch]$NoCargoTools)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$RepoRoot = $PSScriptRoot
$DepsRoot = Join-Path $env:LOCALAPPDATA 'Martensite\deps'
$CargoTools = @('cargo-deny', 'cargo-audit', 'cargo-nextest', 'cargo-llvm-cov')
$WingetPackages = @('Git.Git', 'GitHub.cli', 'Kitware.CMake', 'bloodrock.pkg-config-lite', 'KhronosGroup.VulkanSDK')
# winget returns this when the package is already at the newest version.
$WingetNoApplicableUpdate = -1978335189
# Same sources as the kernel CI (RustyCADKernel .github/workflows/ci.yml); bump them together.
$PinnedSources = @(
    @{ Name = 'tcl'; Repository = 'https://github.com/tcltk/tcl.git'; Commit = '515167c897a7c8066abfbd71e8fe7bd8bf3569de'; Version = '8.6.18' },
    @{ Name = 'tk'; Repository = 'https://github.com/tcltk/tk.git'; Commit = '6dbe68819aac4a22f6f7a143133ce26c5e8d2b05'; Version = '8.6.18' },
    @{ Name = 'freetype'; Repository = 'https://github.com/freetype/freetype.git'; Commit = '0a0221a1347e2f1e07c395263540026e9a0aa7c7'; Version = '2.14.3' }
)

function Write-Step([string]$Message) { Write-Host "==> $Message" }

# Runs a native command and throws, naming it, when it exits non-zero.
function Invoke-Native([string]$Program, [string[]]$Arguments, [int[]]$AllowedExitCodes = @(0)) {
    & $Program @Arguments
    if ($AllowedExitCodes -notcontains $LASTEXITCODE) {
        throw "'$Program $($Arguments -join ' ')' exited with $LASTEXITCODE; expected one of $($AllowedExitCodes -join ', ')"
    }
}

# Runs a native command with its output discarded and returns the exit code.
# Windows PowerShell 5.1 turns redirected stderr into terminating errors
# under ErrorActionPreference 'Stop', so this function relaxes it locally.
function Get-NativeExitCode([string]$Program, [string[]]$Arguments) {
    $ErrorActionPreference = 'Continue'
    & $Program @Arguments *> $null
    return $LASTEXITCODE
}

function Test-Command([string]$Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

# winget and installers change the persisted PATH; reload it into this session.
function Update-SessionPath {
    $machine = [Environment]::GetEnvironmentVariable('Path', 'Machine')
    $user = [Environment]::GetEnvironmentVariable('Path', 'User')
    $env:Path = "$machine;$user;$(Join-Path $env:USERPROFILE '.cargo\bin')"
    foreach ($name in @('VULKAN_SDK', 'PKG_CONFIG_PATH', 'TCL_LIBRARY', 'TK_LIBRARY')) {
        $value = [Environment]::GetEnvironmentVariable($name, 'User')
        if (-not $value) { $value = [Environment]::GetEnvironmentVariable($name, 'Machine') }
        if ($value) { Set-Item "env:$name" $value }
    }
}

# --- winget ------------------------------------------------------------------

function Test-WingetPackage([string]$Id) {
    return (Get-NativeExitCode 'winget' @('list', '--id', $Id, '--exact', '--accept-source-agreements', '--disable-interactivity')) -eq 0
}

function Sync-WingetPackage([string]$Id, [string[]]$InstallArguments = @()) {
    $common = @('--id', $Id, '--exact', '--silent', '--accept-package-agreements', '--accept-source-agreements', '--disable-interactivity')
    if (Test-WingetPackage $Id) {
        Write-Step "Upgrading $Id"
        Invoke-Native 'winget' (@('upgrade') + $common) @(0, $WingetNoApplicableUpdate)
        return
    }
    Write-Step "Installing $Id"
    Invoke-Native 'winget' (@('install') + $common + $InstallArguments)
}

function Assert-Winget {
    if (Test-Command 'winget') { return }
    throw 'winget is missing; expected App Installer from the Microsoft Store (https://aka.ms/getwinget)'
}

# --- MSVC ----------------------------------------------------------------------

function Get-HostArchitecture {
    switch ($env:PROCESSOR_ARCHITECTURE) {
        'AMD64' { return @{ VsArch = 'x64'; Nmake = 'AMD64'; CMake = 'x64'; Component = 'Microsoft.VisualStudio.Component.VC.Tools.x86.x64' } }
        'ARM64' { return @{ VsArch = 'arm64'; Nmake = 'ARM64'; CMake = 'ARM64'; Component = 'Microsoft.VisualStudio.Component.VC.Tools.ARM64' } }
        default { throw "processor architecture '$env:PROCESSOR_ARCHITECTURE' is not supported; expected AMD64 or ARM64" }
    }
}

function Get-VsWhere {
    return Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
}

# Returns the newest Visual Studio installation that has the C++ tools, or $null.
function Find-VisualCpp([hashtable]$Architecture) {
    $vswhere = Get-VsWhere
    if (-not (Test-Path $vswhere)) { return $null }
    $path = & $vswhere -latest -products '*' -requires $Architecture.Component -property installationPath
    if ($path) { return $path }
    return $null
}

# Any Visual Studio edition with the C++ tools is accepted; only a machine
# without one gets the Build Tools with the C++ workload.
function Sync-VisualCpp([hashtable]$Architecture) {
    $override = "--wait --quiet --norestart --add Microsoft.VisualStudio.Workload.VCTools --add $($Architecture.Component) --includeRecommended"
    if ((Find-VisualCpp $Architecture) -and -not (Test-WingetPackage 'Microsoft.VisualStudio.BuildTools')) {
        Write-Step 'Visual Studio with C++ tools found (update it with the Visual Studio Installer)'
        return
    }
    Sync-WingetPackage 'Microsoft.VisualStudio.BuildTools' @('--override', $override)
    if (-not (Find-VisualCpp $Architecture)) {
        throw "Visual Studio has no $($Architecture.Component) after installation; add the 'Desktop development with C++' workload"
    }
}

# Runs a cmd.exe command line inside the Visual Studio developer environment.
function Invoke-DeveloperCommand([hashtable]$Architecture, [string]$CommandLine) {
    $devCmd = Join-Path (Find-VisualCpp $Architecture) 'Common7\Tools\VsDevCmd.bat'
    Invoke-Native 'cmd.exe' @('/d', '/c', "call `"$devCmd`" -arch=$($Architecture.VsArch) -no_logo && $CommandLine")
}

# --- Rust ----------------------------------------------------------------------

function Sync-Rustup {
    if (Test-Command 'rustup') {
        Write-Step 'rustup found; updating it'
        Invoke-Native 'rustup' @('self', 'update')
        return
    }
    Sync-WingetPackage 'Rustlang.Rustup'
    Update-SessionPath
}

# Reads `key = "value"` or `key = ["a", "b"]` from rust-toolchain.toml.
function Get-ToolchainSetting([string]$Key) {
    $line = Get-Content (Join-Path $RepoRoot 'rust-toolchain.toml') | Where-Object { $_ -match "^$Key\s*=" } | Select-Object -First 1
    if (-not $line) { throw "no '$Key' in rust-toolchain.toml; expected $Key = ..." }
    return ($line -replace "^$Key\s*=\s*", '') -replace '[\[\]" ]', ''
}

function Install-PinnedToolchain {
    $channel = Get-ToolchainSetting 'channel'
    $components = Get-ToolchainSetting 'components'
    Write-Step "Installing Rust $channel with $components and llvm-tools-preview"
    # llvm-tools-preview is what cargo-llvm-cov needs for the coverage gate.
    Invoke-Native 'rustup' @('toolchain', 'install', $channel, '--profile', 'minimal', '--component', "$components,llvm-tools-preview")
}

function Sync-CargoToolList {
    $installed = (& cargo install --list) -join "`n"
    foreach ($tool in $CargoTools) {
        $action = if ($installed -match "(?m)^$tool v") { 'Updating' } else { 'Installing' }
        Write-Step "$action $tool"
        # `cargo install` rebuilds only when crates.io has a newer version.
        Push-Location $RepoRoot
        try { Invoke-Native 'cargo' @('install', '--locked', $tool) } finally { Pop-Location }
    }
}

# --- Pinned native libraries ---------------------------------------------------

# Fetches one pinned commit into a clean source directory and returns its path.
function Get-PinnedSource([hashtable]$Source) {
    $directory = Join-Path $DepsRoot "src\$($Source.Name)"
    if (Test-Path $directory) { Remove-Item -Recurse -Force $directory }
    Invoke-Native 'git' @('init', '--quiet', $directory)
    Invoke-Native 'git' @('-C', $directory, 'fetch', '--quiet', '--depth', '1', $Source.Repository, $Source.Commit)
    Invoke-Native 'git' @('-C', $directory, 'checkout', '--quiet', 'FETCH_HEAD')
    return $directory
}

# A marker records the commit each library was built from; a matching marker
# means the library is current and is not rebuilt.
function Test-BuiltSource([hashtable]$Source) {
    $marker = Join-Path $DepsRoot "$($Source.Name).commit"
    return (Test-Path $marker) -and ((Get-Content $marker -Raw).Trim() -eq $Source.Commit)
}

function Set-BuiltSource([hashtable]$Source) {
    Set-Content -Encoding ascii (Join-Path $DepsRoot "$($Source.Name).commit") $Source.Commit
}

function Build-TclTk([hashtable]$Architecture, [string]$Prefix) {
    $tcl, $tk = $PinnedSources[0], $PinnedSources[1]
    if ((Test-BuiltSource $tcl) -and (Test-BuiltSource $tk)) {
        Write-Step "Tcl/Tk $($tcl.Version) is current"
        return
    }
    Write-Step "Building Tcl/Tk $($tcl.Version) with MSVC"
    $tclSource = Get-PinnedSource $tcl
    $tkSource = Get-PinnedSource $tk
    foreach ($source in @($tclSource, $tkSource)) {
        $nmake = "nmake -nologo -f makefile.vc release install MACHINE=$($Architecture.Nmake) `"INSTALLDIR=$Prefix`" `"TCLDIR=$tclSource`""
        Invoke-DeveloperCommand $Architecture "cd /d `"$source\win`" && $nmake"
    }
    Write-TclPkgConfig $Prefix $tcl.Version
    Set-BuiltSource $tcl
    Set-BuiltSource $tk
}

# The Windows makefiles install no pkg-config metadata; describe the built
# threaded import libraries under the module names the kernel asks for.
function Write-TclPkgConfig([string]$Prefix, [string]$Version) {
    $directory = Join-Path $Prefix 'lib\pkgconfig'
    New-Item -ItemType Directory -Force $directory | Out-Null
    foreach ($module in @('tcl', 'tk')) {
        $library = Join-Path $Prefix "lib\${module}86t.lib"
        if (-not (Test-Path $library)) { throw "missing $library; expected the threaded MSVC import library" }
        $text = "prefix=$($Prefix.Replace('\', '/'))`nName: ${module}8.6`nDescription: Martensite Tcl/Tk built with MSVC`n" +
            "Version: $Version`nLibs: -L`${prefix}/lib -l${module}86t`nCflags: -I`${prefix}/include`n"
        Set-Content -Encoding ascii (Join-Path $directory "${module}8.6.pc") $text
    }
}

# Optional codecs are off: the kernel loads FreeType for outline fonts only,
# and the bundled inflate still covers compressed fonts.
function Build-FreeType([hashtable]$Architecture, [string]$Prefix) {
    $freetype = $PinnedSources[2]
    if (Test-BuiltSource $freetype) {
        Write-Step "FreeType $($freetype.Version) is current"
        return
    }
    Write-Step "Building FreeType $($freetype.Version) with MSVC"
    $source = Get-PinnedSource $freetype
    $build = Join-Path $DepsRoot 'build\freetype'
    $options = '-DBUILD_SHARED_LIBS=ON -DFT_DISABLE_ZLIB=ON -DFT_DISABLE_BZIP2=ON -DFT_DISABLE_PNG=ON -DFT_DISABLE_HARFBUZZ=ON -DFT_DISABLE_BROTLI=ON'
    Invoke-DeveloperCommand $Architecture "cmake -S `"$source`" -B `"$build`" -A $($Architecture.CMake) $options `"-DCMAKE_INSTALL_PREFIX=$Prefix`""
    Invoke-DeveloperCommand $Architecture "cmake --build `"$build`" --config Release --parallel && cmake --install `"$build`" --config Release"
    if (-not (Test-Path (Join-Path $Prefix 'bin\freetype.dll'))) { throw "missing $Prefix\bin\freetype.dll after the FreeType build" }
    Set-BuiltSource $freetype
}

# --- User environment ----------------------------------------------------------

# Edits the registry value directly: [Environment]::SetEnvironmentVariable
# would expand `%VAR%` entries in the user's PATH and store them as plain text.
function Add-UserPathEntry([string]$Name, [string]$Entry) {
    $key = Get-Item 'HKCU:\Environment'
    $current = $key.GetValue($Name, '', 'DoNotExpandEnvironmentNames')
    $entries = @($current -split ';' | Where-Object { $_ })
    if ($entries -contains $Entry) { return }
    $value = (@($Entry) + $entries) -join ';'
    New-ItemProperty -Path 'HKCU:\Environment' -Name $Name -Value $value -PropertyType ExpandString -Force | Out-Null
}

function Set-MartensiteEnvironment([string]$TclPrefix, [string]$FreeTypePrefix) {
    Add-UserPathEntry 'PKG_CONFIG_PATH' (Join-Path $TclPrefix 'lib\pkgconfig')
    Add-UserPathEntry 'Path' (Join-Path $TclPrefix 'bin')
    Add-UserPathEntry 'Path' (Join-Path $FreeTypePrefix 'bin')
    # These two calls also broadcast the change, so new terminals see the
    # registry edits above without a sign-out.
    [Environment]::SetEnvironmentVariable('TCL_LIBRARY', (Join-Path $TclPrefix 'lib\tcl8.6'), 'User')
    [Environment]::SetEnvironmentVariable('TK_LIBRARY', (Join-Path $TclPrefix 'lib\tk8.6'), 'User')
    Update-SessionPath
}

function Assert-Installation {
    $commands = @('git', 'gh', 'cmake', 'pkg-config', 'rustup', 'cargo', 'glslangValidator', 'spirv-val', 'vulkaninfo')
    $missing = @($commands | Where-Object { -not (Test-Command $_) })
    if ((Get-NativeExitCode 'pkg-config' @('--exists', 'tcl8.6', 'tk8.6')) -ne 0) { $missing += 'pkg-config modules tcl8.6 tk8.6' }
    if ($missing.Count -gt 0) { throw "missing after installation: $($missing -join ', ')" }
    Show-VulkanDeviceList
}

function Show-VulkanDeviceList {
    $ErrorActionPreference = 'Continue'
    Write-Step 'Vulkan devices:'
    $devices = & vulkaninfo --summary 2> $null | Select-String 'deviceName|driverName'
    if (-not $devices) { throw 'vulkaninfo found no Vulkan device; expected a GPU driver with Vulkan support' }
    $devices | ForEach-Object { Write-Host $_.Line }
}

function Main {
    Assert-Winget
    $architecture = Get-HostArchitecture
    Sync-VisualCpp $architecture
    foreach ($id in $WingetPackages) { Sync-WingetPackage $id }
    Update-SessionPath
    Sync-Rustup
    Install-PinnedToolchain
    if (-not $NoCargoTools) { Sync-CargoToolList }
    $tclPrefix = Join-Path $DepsRoot 'tcltk'
    $freetypePrefix = Join-Path $DepsRoot 'freetype'
    Build-TclTk $architecture $tclPrefix
    Build-FreeType $architecture $freetypePrefix
    Set-MartensiteEnvironment $tclPrefix $freetypePrefix
    Assert-Installation
    Write-Step 'Done. Open a new terminal to use the updated environment.'
}

Main
