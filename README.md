# rsync-win

`rsync` for Windows.

see [releases]() page for binaries.

I had a bad time trying to find a working `rsync` for Windows. I read some blogs about [cwRsync](https://www.itefix.net/cwrsync), but I just couldn't get it to work.

Finally, I repacked the `rsync.exe`, `ssh.exe` and `cygpath.exe` from Cygwin, and wrote a `Rust` wrapper around it.

It works for me, and I hope it works for you too.

# Usage

- e.g. bandwidth limit `2048 KBytes/s`, `-a`, `-v`, show progress, copy from remote machine to local, exclude `*.log` files, and save to `./target/` directory.

  ```
  rsync-win --bwlimit=2048 -av --exclude='*.log' --progress -s <REMOTE USER>@<REMOTE_MACHINE>:<REMOTE_PATH> -d ./target/
  ```

- Help

  ```
  rsync for Windows.
  Most of the options are the same as the original rsync.

  Usage: rsync-win.exe [OPTIONS] --src <SRC> --dest <DEST>

  Options:
    -i, --identity <IDENTITY>  SSH identity file [default: "C:/Users/<YOUR USER NAME>/.ssh/id_rsa"]
    -v, --verbose
    -q, --quiet
    -c, --checksum
    -a, --archive
    -r, --recursive
        --delete
        --exclude <EXCLUDE>
        --progress
        --bwlimit <BWLIMIT>
    -4, --ipv4
    -6, --ipv6
    -s, --src <SRC>
    -d, --dest <DEST>
    -h, --help                 Print help
  ```

some blogs:

- [如何讓 OpenSSH for Windows 也能支援 Rsync 遠端加密連線傳輸](https://blog.miniasp.com/post/2021/12/15/How-to-use-Rsync-with-OpenSSH-for-Windows)

- [如何在 Windows 安裝 Cygwin 的 SSHD 服務並正確使用 Rsync 同步檔案](https://blog.miniasp.com/post/2021/12/13/How-to-use-Cygwin-cygsshd-and-rsync-in-Windows)
