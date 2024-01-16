<#
How to use?
Execute this powershell file by double-clicking it in the root of the mod (where mesh, etc. are located).

PowerShell files written on someone else's PC may be blocked for security reasons.
In that case, you can create a new file by yourself and copy the contents of this file.
#>

$target = "." # target dir(. == current dir)
$mappingFile = "mapping_table.txt"
$renameFrom = "" # Regexp
$renameTo = ""

# Search for the path containing the "_CustomConditions" directory
$dirPath = $(Get-ChildItem -Path $target -Directory -Recurse -Filter "_CustomConditions").FullName

# Get the list of directories
$directories = Get-ChildItem -Path $dirPath -Recurse -Filter '*.txt' | Sort-Object Name

# Initialize an array to store the results
$results = @()
foreach ($dir in $directories)
{
  $number = Split-Path $dir -Parent | Split-Path -Leaf # Get DAR priority dirname
  if ($dir.Name -ne '_condition.txt')
  {
        $name = $dir.BaseName -replace $renameFrom, $renameTo
        $results += "$number $name"
  }
}

# Write the results to the mapping_table.txt file in UTF-8 encoding
$results | Out-File -FilePath $mappingFile -Encoding utf8

Write-Host $results
Write-Host "Mapping table has been written to $mappingFile"
