# rsync-win

`rsync` for Windows.

See [releases](https://github.com/rn7s2/rsync-win/releases) page for binaries.

I had a bad time trying to find a working `rsync` for Windows. I read some blogs about [cwRsync](https://www.itefix.net/cwrsync), but I just couldn't get it to work.

Finally, I repacked the `rsync.exe`, `ssh.exe` and `cygpath.exe` from Cygwin, and wrote a `Rust` wrapper around it.

It works for me, and I hope it works for you too.

# Installation

1. Goto [releases](https://github.com/rn7s2/rsync-win/releases) page & download `rsync-win.zip`.
2. Extract. Keep `cygwin64` folder along side with `rsync-win.exe`.
3. Add the directory of `rsync-win.exe` to `PATH`.
4. Restart your terminal and try executing `rsync-win -h`.

# Usage

- bandwidth limit `2048 KBytes/s`, `-a`, `-v`, show progress, copy from remote machine to local, exclude `*.log` files, and save to `./target/` directory.

  ```
  rsync-win --bwlimit=2048 -av --exclude='*.log' --progress -s <REMOTE USER>@<REMOTE_MACHINE>:<REMOTE_PATH> -d ./target/
  ```

- Help

  ```
  Rsync for Windows

  Allowed formats for <SRC>/<DEST>:
    local: C:/path/to/file
      ssh: [USER@]HOST:/path/to/file (use --ssh-port to specify the port)
    rsync: rsync://[USER@]HOST[:PORT]/path/to/file


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
        --partial
        --progress
        --bwlimit <BWLIMIT>
    -4, --ipv4
    -6, --ipv6
        --ssh-port <SSH_PORT>
    -s, --src <SRC>
    -d, --dest <DEST>
    -h, --help                 Print help
    -V, --version              Print version
  ```

Some blogs:

- [如何讓 OpenSSH for Windows 也能支援 Rsync 遠端加密連線傳輸](https://blog.miniasp.com/post/2021/12/15/How-to-use-Rsync-with-OpenSSH-for-Windows)

- [如何在 Windows 安裝 Cygwin 的 SSHD 服務並正確使用 Rsync 同步檔案](https://blog.miniasp.com/post/2021/12/13/How-to-use-Cygwin-cygsshd-and-rsync-in-Windows)
