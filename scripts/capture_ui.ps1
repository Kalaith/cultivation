<#
.SYNOPSIS
    Headless screenshot harness for Cultivation (Heavenly Mandate).

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (CULTIVATION_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Scene names map to Game::begin_capture_scene arms: "mainmenu"
    (default boot state), "sectbase" (home base management screen), and
    "worldmap" (outside route map).

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [string[]]$Scenes = @("mainmenu", "sectbase", "worldmap"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
