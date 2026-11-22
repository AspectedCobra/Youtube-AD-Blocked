# Downloads the latest uBlock Origin Lite (Chromium build) into assets\ubol.zip
$rel = Invoke-RestMethod "https://api.github.com/repos/uBlockOrigin/uBOL-home/releases/latest"
$asset = $rel.assets | Where-Object { $_.name -like "*chromium*.zip" } | Select-Object -First 1
if (-not $asset) { throw "Could not find the uBOL chromium zip in the latest release" }
New-Item -ItemType Directory -Force assets | Out-Null
Invoke-WebRequest $asset.browser_download_url -OutFile assets\ubol.zip
Write-Host "Downloaded $($asset.name)"
