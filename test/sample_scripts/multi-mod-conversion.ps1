function Convert-Mods($base, $mods_dir, $log_level) {
  <# Example dir status
D:/Programming/rust/dar-to-oar
  ├─── test
  │     └─── data
  │           ├─── Modern Female Sitting Animations Overhaul
  │           └─── UNDERDOG Animations
  └─── logs
#>

  # Create log dir if it doesn't exist.
  if (!$(Test-Path "$base_dir/logs")) {
    New-Item -ItemType Directory "$base_dir/logs"
  }

  Get-ChildItem $mods_dir -Directory |
  ForEach-Object {
    # The following values are expected for `$_.FullName`.
    # - D:/Programming/rust/dar-to-oar/test/data/Modern Female Sitting Animations Overhaul
    # - D:/Programming/rust/dar-to-oar/test/data/UNDERDOG Animations

    # The following values are expected for `$_.Name`.
    # - Modern Female Sitting Animations Overhaul
    # - UNDERDOG Animations
    cargo run --release -- `
      convert $_.FullName `
      --run-parallel `
      --stdout `
      --log-level $log_level `
      --log-path "$base_dir/logs/convert-$($_.Name).log"
    Write-Host ""
  }
}

function Show-Dar($base, $mods_dir, $log_level) {
  if (!$(Test-Path "$base_dir/logs")) {
    New-Item -ItemType Directory "$base_dir/logs"
  }

  Get-ChildItem $mods_dir -Directory |
  ForEach-Object {
    cargo run --release -- `
      unhide-dar $_.FullName `
      --stdout `
      --log-level $log_level `
      --log-path "$base_dir/logs/convert-$($_.Name).log"
    Write-Host ""
  }
}

function Remove-Oar($base, $mods_dir, $log_level) {
  if (!$(Test-Path "$base_dir/logs")) {
    New-Item -ItemType Directory "$base_dir/logs"
  }

  Get-ChildItem $mods_dir -Directory |
  ForEach-Object {
    cargo run --release -- `
      remove-oar $_.FullName `
      --stdout `
      --log-level $log_level `
      --log-path "$base_dir/logs/convert-$($_.Name).log"
    Write-Host ""
  }
}
function Get-Help() {
  cargo run --release -- --help
  cargo run --release -- convert --help
  cargo run --release -- remove-oar --help
  cargo run --release -- unhide-dar --help
}

$base_dir = "D:/Programming/rust/dar-to-oar" # Convert target base directory
# Convert-Mods $base_dir "$base_dir/test/data" "debug"
Remove-Oar  $base_dir "$base_dir/test/data" "debug"
Show-Dar $base_dir "$base_dir/test/data" "debug"
# Get-Help
