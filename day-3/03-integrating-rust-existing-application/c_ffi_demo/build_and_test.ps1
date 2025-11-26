# PowerShell build script for Windows (MSVC)

Write-Host "Building Rust library..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
    
    Write-Host "`nCompiling C test program..." -ForegroundColor Cyan
    
    # Find the library file
    $libPath = "target\release"
    $libFile = "$libPath\mathlib.lib"
    $dllFile = "$libPath\mathlib.dll"
    
    if (Test-Path $libFile) {
        # Compile with MSVC
        cl examples\test.c /I. /Fe:examples\test.exe /link $libFile
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Compilation successful!" -ForegroundColor Green
            
            # Copy DLL to examples directory for execution
            if (Test-Path $dllFile) {
                Copy-Item $dllFile examples\
            }
            
            Write-Host "`nRunning test program..." -ForegroundColor Cyan
            Write-Host "========================================" -ForegroundColor Yellow
            & examples\test.exe
            Write-Host "========================================" -ForegroundColor Yellow
        } else {
            Write-Host "Compilation failed!" -ForegroundColor Red
        }
    } else {
        Write-Host "Library file not found: $libFile" -ForegroundColor Red
        Write-Host "Available files in $libPath:" -ForegroundColor Yellow
        Get-ChildItem $libPath
    }
} else {
    Write-Host "Build failed!" -ForegroundColor Red
}
