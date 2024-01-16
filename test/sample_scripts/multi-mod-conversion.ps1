<# Example dir status
D:/Programming/rust/dar-to-oar
  ├─── test
  │     └─── data
  │           ├─── Modern Female Sitting Animations Overhaul
  │           └─── UNDERDOG Animations
  └─── logs
#>

cargo build --release

$base_dir = "D:/Programming/rust/dar-to-oar" # Convert target base directory
$bin_dir = "D:/Programming/rust/dar-to-oar/target/release" # Directory with dar2oar.exe

if (!$(Get-Command dar2oar -ErrorAction SilentlyContinue)) {
  # Temporarily pass through to access without specifying an absolute path.
  $env:Path += ";$bin_dir"
}

function Convert-Mods($base, $mods_dir, $log_level) {
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
    dar2oar convert $_.FullName `
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
    dar2oar unhide-dar $_.FullName `
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
    dar2oar remove-oar $_.FullName `
      --stdout `
      --log-level $log_level `
      --log-path "$base_dir/logs/convert-$($_.Name).log"
    Write-Host ""
  }
}

function Get-Help() {
  dar2oar --help
  dar2oar --help
  dar2oar convert --help
  dar2oar remove-oar --help
  dar2oar unhide-dar --help
}

# Convert-Mods $base_dir "$base_dir/test/data" "debug"
# Remove-Oar  $base_dir "$base_dir/test/data" "debug"
# Show-Dar $base_dir "$base_dir/test/data" "debug"
Get-Help
