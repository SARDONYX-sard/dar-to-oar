<#
How to use?
Execute this powershell file by double-clicking it in the root of the mod (where mesh, etc. are located).

PowerShell files written on someone else's PC may be blocked for security reasons.
In that case, you can create a new file by yourself and copy the contents of this file.
#>

$target = "." # target dir(. == current dir)
$mappingFile = "mapping_table.txt"

# Search for the path containing the "_CustomConditions" directory
$dirPath = $(Get-ChildItem -Path $target -Directory -Recurse -Filter "_CustomConditions").FullName

# Get the list of directories
$directories = Get-ChildItem -Path $dirPath -Directory | Sort-Object Name

# Define a regular expression pattern to extract the desired information
$pattern = '(\d+)\s*-\s*(.+)'

# Initialize an array to store the results
$results = @()

# Process each directory
foreach ($directory in $directories) {
    $name = $directory.Name
    if ($name -match $pattern) {
        $number = $matches[1]
        $name = $matches[2]
        $results += "$number $name"
    } else {
        $results += $directory.Name
    }
}

# Write the results to the mapping_table.txt file in UTF-8 encoding
$results | Out-File -FilePath $mappingFile -Encoding utf8

Write-Host "Mapping table has been written to $mappingFile"
