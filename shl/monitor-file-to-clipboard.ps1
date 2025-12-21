param(
    [Parameter(Mandatory=$true)]
    [string]$FilePath,
    
    [int]$CheckInterval = 1000
)

Write-Host "Monitoring file: $FilePath"
Write-Host "Check interval: $CheckInterval ms"
Write-Host "Press Ctrl+C to stop monitoring..."
Write-Host ""

# Initialize last file size
$lastSize = 0

while ($true) {
    try {
        if (Test-Path -Path $FilePath -PathType Leaf) {
            $fileInfo = Get-Item -Path $FilePath
            $currentSize = $fileInfo.Length
            
            # Only copy when file size is greater than 0 and different from last time
            if ($currentSize -gt 0 -and $currentSize -ne $lastSize) {
                Write-Host "[$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')] File has new content, copying to clipboard..."
                $content = Get-Content -Path $FilePath -Raw
                Set-Clipboard -Value $content
                Write-Host "Content copied to clipboard"
                $lastSize = $currentSize
            }
        } else {
            Write-Host "[$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')] File not found: $FilePath"
        }
    } catch {
        Write-Host "[$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')] Error: $($_.Exception.Message)"
    }
    
    # Wait for the specified interval
    Start-Sleep -Milliseconds $CheckInterval
}