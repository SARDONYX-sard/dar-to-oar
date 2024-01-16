# Setup test mods

At this time, automatic installation is not possible and must be downloaded
manually.

```powershell
<# function Get-Mod([string]$dist, [string]$url) {
  Invoke-WebRequest $url -OutFile "$dist.zip"

  7zip "$dist.zip" -DestinationPath $dist
  # Remove-Item "$dist.zip"
} #>

@(
  @(
    $dist = 'Animated Armoury DAR Modified Conditions'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/74320?tab=files&file_id=395456'
  ),
  @(
    $dist = 'Modern Female Sitting Animations Overhaul'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/85599?tab=files&file_id=444438'
  ),
  @(
    $dist = 'UNDERDOG - Animations'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/51811?tab=files&file_id=461119'
  )
)
# Get-Mod $dist $url
```
