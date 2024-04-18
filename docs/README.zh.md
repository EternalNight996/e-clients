<img src="../public/ico/white_64x64.ico" alt="e-utils"/>

### 📄 [中文](README.zh.md)  | 📄  [English](../README.md)

# ⚡ 这是什么?
**这是一个支持SSH | FTP | SMB 等传输协议的库**

### 支持 功能
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff"><center>功能</center></h3></th>
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

# ✨ 分支
```toml
[features]
ftp = ["suppaftp", "chrono"]
smb = ["remotefs-smb"]
ssh = ["remotefs-ssh"]
default = []
```

# 📖 示例
```toml
[dependencies]
e-clients = {version="0.1", feature=["ftp","ssh","smb"]}
```

### Examples
[SSH Example](examples/ssh.rs) | [FTP Example](examples/ftp.rs) | [SMB Example](examples/smb.rs)


## `💡!重要：`


# 🚀 快速运行
```sh
# test ftp
cargo run --example ftp

# test ssh
cargo run --example ssh

# test smb
cargo run --example smb
```


# 🦊 已运用项目
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): 网络扫描项目（同时支持命令行与跨平台图形化界面）正在开发中。。 -->

# 🔭 为什么需要e-utils?
<!-- 起初是想完成一个跨网络扫描项目，帮助自己完成一些工作，参考许多开源项目,但这些项目多少有些缺陷并不满足自己需求，所以有了e-libscanner。
(处理主机和端口扫描，同时支持域名解析、路由跟踪、指纹扫描、服务扫描、异步扫描、可扩展更多)
底层是通过调用[npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)抓包服务；
服务api为[libpnet](https://github.com/libpnet/libpnet); -->

# 🙋 参考项目与资料
<!-- ✨[RustScan](https://github.com/RustScan/RustScan) :Rust仿nmap扫描库
✨[netscan](https://github.com/shellrow/netscan) :Rust 网络扫描库
✨[libpnet](https://github.com/libpnet/libpnet) 跨平台网络底层库--主要是调用抓包服务([npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)) -->