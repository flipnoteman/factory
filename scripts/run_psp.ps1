# Set Environment Variables
$RUN = "factory.prx"
$TARGET_DIR = "\target\mipsel-sony-psp\debug\"
cargo psp

#start "" "usbhostfs_pc.exe"
$usbhost_process = Start-Process usbhostfs_pc.exe -ArgumentList $TARGET_DIR -PassThru
Write-Output "usbhostfs_pc.exe started at $TARGET_DIR."
Start-Sleep -Milliseconds 600
Start-Process pspsh -ArgumentList "-e", $RUN
Write-Output "$RUN running..."
Wait-Process -Id $usbhost_process.Id
Write-Output "usbhostfs_pc.exe stopped. Exiting..."
pspsh -e "reset"
Write-Output "Psplink successfully restarted"

