$CWD = Get-Location

$files = Get-ChildItem -Path $CWD

if ($files.Name -Contains 'Cargo.toml') {
    Write-Output "Cargo.toml found in $CurrentDirectory"
} else {
    $ParentDir = (Get-Item $CWD).Parent.FullName
    Write-Output "Using parent directory: $ParentDir"
    $CWD = Set-Location -Path $ParentDir -PassThru
}


$EBOOT = $CWD.Path + "\target\mipsel-sony-psp\debug\EBOOT.PBP"

cargo psp
Write-Output "Loading: $($EBOOT)"

$firstPath = "C:\Program Files (x86)\PPSSPPGold\PPSSPPWindows.exe"
$secondPath = "C:\Program Files\PPSSPP\PPSSPPWindows.exe"

# Check if the first path exists
if (Test-Path $firstPath) {
    Write-Output "Starting PPSSPP from first path."
    Start-Process $firstPath -ArgumentList "--load", $EBOOT
    Write-Output "Opening PPSSPP ..."
} elseif (Test-Path $secondPath) {
    Write-Output "Failed to find PPSSPP at: $firstPath."
    Start-Process $secondPath -ArgumentList "--load", $EBOOT
    Write-Output "Opening PPSSPP ..."
} else {
    Write-Output "Failed to find PPSSPP at: $secondPath."
    Write-Output "Neither of the specified PPSSPP paths exist. Please check make sure PPSSPP is installed."
}

