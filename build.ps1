param([switch]$Test)
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
try {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw 'Instala Rust con rustup. Consulta el README.'
    }
    cargo build --release --locked
    if ($LASTEXITCODE -ne 0) { throw 'No se pudo compilar el proyecto Rust.' }
    if ($Test) {
        cargo test --locked
        if ($LASTEXITCODE -ne 0) { throw 'Fallo una prueba.' }
    }
    New-Item -ItemType Directory -Force -Path bin | Out-Null
    Copy-Item -LiteralPath target\release\carga_modelos.exe -Destination bin\carga_modelos.exe -Force
    Copy-Item -LiteralPath vendor\SDL2\lib\x64\SDL2.dll -Destination bin\SDL2.dll -Force
    Copy-Item -LiteralPath vendor\SDL2\LICENSE.txt -Destination bin\SDL2-LICENSE.txt -Force
    Write-Host 'Listo. Ejecuta .\bin\carga_modelos.exe'
} finally { Pop-Location }
