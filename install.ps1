# Set the download URL and target path
$downloadUrl = "https://github.com/Timmoth/aid-cli/releases/download/aid-0.1.0/aid-win.exe"
$installDir = "C:\Program Files\AidCLI"
$filename = "aid.exe"

# Create the installation directory if it doesn't exist
if (-not (Test-Path -Path $installDir)) {
    Write-Host "Creating install directory..."
    New-Item -ItemType Directory -Path $installDir
}

# Download the file using Invoke-WebRequest
$destination = Join-Path -Path $installDir -ChildPath $filename
Write-Host "Downloading aid-win.exe..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $destination

if (-not (Test-Path -Path $destination)) {
    Write-Host "Download failed! File not found."
    exit 1
}

Write-Host "Download successful! File saved to $destination"

# Check if the installation directory is already in the PATH
$envPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
if ($envPath -notlike "*$installDir*") {
    Write-Host "Adding $installDir to system PATH..."

    # Add the directory to the PATH
    $newPath = "$envPath;$installDir"
    [System.Environment]::SetEnvironmentVariable("Path", $newPath, [System.EnvironmentVariableTarget]::Machine)

    Write-Host "$installDir successfully added to system PATH"
} else {
    Write-Host "$installDir is already in the system PATH"
}

Write-Host "Installation complete! You can now use the 'aid' command from any terminal."
