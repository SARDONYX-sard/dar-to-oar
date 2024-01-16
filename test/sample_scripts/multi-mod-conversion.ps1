<#

Example dir status

D:/Programming/rust/dar-to-oar
  ├─── test
  │     └─── data
  │           ├─── Modern Female Sitting Animations Overhaul
  │           └─── UNDERDOG Animations
  └─── logs

#>
# Convert target base directory
$base_dir = "D:/Programming/rust/dar-to-oar"

# Create log dir if it doesn't exist.
if (!$(Test-Path "$base_dir/logs")) {
  New-Item -ItemType Directory "$base_dir/logs"
}

Get-ChildItem "$base_dir/test/data" |
ForEach-Object {
  # The following values are expected for `$_.FullName`.
  # - D:/Programming/rust/dar-to-oar/test/data/Modern Female Sitting Animations Overhaul
  # - D:/Programming/rust/dar-to-oar/test/data/UNDERDOG Animations

  # The following values are expected for `$_.Name`.
  # - Modern Female Sitting Animations Overhaul
  # - UNDERDOG Animations
  cargo run --release -- `
    --src $_.FullName `
    --run-parallel `
    --log-level "info" `
    --log-path "$base_dir/logs/convert-$($_.Name).log"
}
