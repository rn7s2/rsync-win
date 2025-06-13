$dlls = (Get-ChildItem *.dll)

for ($i = 0; $i -lt $dlls.Length; $i++) {
    Write-Host $dlls[$i].Name
    Move-Item $dlls[$i].Name "test/$($dlls[$i].Name)"

    ./cygpath.exe -V
    $ok1 = ($LASTEXITCODE -eq 0)

    ./rsync.exe -V
    $ok2 = ($LASTEXITCODE -eq 0)

    ./ssh.exe -V
    $ok3 = ($LASTEXITCODE -eq 0)

    if ($ok1 -and $ok2 -and $ok3) {
        Write-Host "it's ok to remove $($dlls[$i].Name)"
    } else {
        Move-Item "test/$($dlls[$i].Name)" $dlls[$i].Name
    }
}
