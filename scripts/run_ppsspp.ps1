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
Start-Process "C:\Program Files (x86)\PPSSPPGold\PPSSPPWindows.exe" -ArgumentList "--load", $EBOOT
