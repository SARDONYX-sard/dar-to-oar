# Setup test mods

At this time, automatic installation is not possible and must be downloaded
manually.

```powershell
<# function Get-Mod([string]$dst, [string]$url) {
  Invoke-WebRequest $url -OutFile "$dst.zip"

  7zip "$dst.zip" -DstinationPath $dst
  # Remove-Item "$dst.zip"
} #>

@(
  @(
    $dst = 'Animated Armoury DAR Modified Conditions'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/74320?tab=files&file_id=395456'
  ),
  @(
    $dst = 'Modern Female Sitting Animations Overhaul'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/85599?tab=files&file_id=444438'
  ),
  @(
    $dst = 'UNDERDOG - Animations'
    $url = 'https://www.nexusmods.com/skyrimspecialedition/mods/51811?tab=files&file_id=461119'
  )
)
# Get-Mod $dst $url
```
