# Compilar el proyecto en modo debug
Write-Host "Compilando el proyecto..."
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error durante la compilación. Saliendo..."
    exit $LASTEXITCODE
}

# Ejecutar el binario compilado en una nueva ventana de consola
Write-Host "Ejecutando el programa en una nueva ventana de consola..."
Start-Process "cmd.exe" -ArgumentList "/c start cmd.exe /k .\src\main.exe"
