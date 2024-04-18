<img src="public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](docs/README.zh.md)  | 📄  [English](README.md)

# ⚡ what this ?
**This is a library that supports transmission protocols such as SSH, FTP, and SMB**

### Support APP
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff"><center>APP</center></h3></th>
    <th><h3 style="color:#fff"><center>Windows10</center></h3></th>
    <th><h3 style="color:#fff"><center>Unix</center></h3></th>
    <th><h3 style="color:#fff"><center>MacOs</center></h3></th>
    <th><h3 style="color:#fff"><center>-</center></h3></th>
  </tr>
  <tr>
    <td><center>SSH</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>SSH Client</center></h4></td>
  </tr>
  <tr>
    <td><center>FTP</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>FTP Client</center></h4></td>
  </tr>
  <tr>
    <td><center>SMB</center></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:green"><center>√</center></h4></td>
    <td><h4 style="color:red"><center>X</center></h4></td>
    <td><h4 style="color:#aaa"><center>SMB Client</center></h4></td>
  </tr>
</table>

# ✨ Features
```toml
[features]
ftp = ["suppaftp", "chrono"]
smb = ["remotefs-smb"]
ssh = ["remotefs-ssh"]
default = []
```

# 📖 Example
```toml
[dependencies]
e-clients = {version="0.1", feature=["ftp","ssh","smb"]}
```

### Examples
[SSH Example](examples/ssh.rs) | [FTP Example](examples/ftp.rs) | [SMB Example](examples/smb.rs)


## `💡!important：`

# 🚀 fast running
```sh
# test ftp
cargo run --example ftp

# test ssh
cargo run --example ssh

# test smb
cargo run --example smb
```

# 🦊 Applied Projects
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): The network scanning project (which supports both command line and cross platform graphical interface) is under development.. -->

# 🔭 why need to e-clients?
<!-- At first, I wanted to complete a cross network scanning project to help me complete some work. I referred to many open source projects, but these projects have some defects that do not meet my needs, so I have e-libscanner.
(process host and port scanning, and support domain name resolution, route tracking, fingerprint scanning, service scanning, asynchronous scanning, scalability and more)
The bottom layer is by calling [npcap]（ https://nmap.org/npcap/ ）And [WinPcap]（ https://www.winpcap.org/ ）Packet capture service;
The service API is [libpnet]（ https://github.com/libpnet/libpnet ); -->

# 🙋 Reference items and materials
<!-- ✨ [RustScan] https://github.com/RustScan/RustScan : Rustlike nmapscan
✨ [netscan] https://github.com/shellrow/netscan : Rust Network Scanner
✨ [libpnet](libpnet) https://github.com/libpnet/libpnet ● The background base of the interplatform network - mainly using captivity services ([npcap]) https://nmap.org/npcap/ with [WinPcap]( https://www.winpcap.org/ ) -->