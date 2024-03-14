# nc-rust

nc-rust 是一个简单的类似于 nc 工具的 TCP 客户端/服务器实用程序，使用 Rust 编写。
## 用法
作为服务器模式运行

要将 nc-rust 设置为服务器模式，并监听指定的本地端口，请使用以下命令：


    ./nc-rust -l <LOCAL_PORT>

其中 <LOCAL_PORT> 是要监听的本地端口号。
作为客户端模式运行

要将 nc-rust 设置为客户端模式，并连接到指定的远程主机和端口，请使用以下命令：


    ./nc-rust -u <REMOTE_HOST> -p <REMOTE_PORT>

其中 <REMOTE_HOST> 是要连接的远程主机的 IP 地址或主机名， <REMOTE_PORT> 是要连接的远程端口号。
命令行选项

    -l, --local-port <LOCAL_PORT>: 设置本地端口并启动服务器模式。
    -u, --remote-host <REMOTE_HOST>: 设置远程主机以连接到，并结合 -p 选项使用。
    -p, --remote-port <REMOTE_PORT>: 设置要连接的远程端口，并结合 -u 选项使用。

示例

    在服务器模式下监听本地端口 8888：


    ./nc-rust -l 8888

    在客户端模式下连接到远程主机 192.168.2.32 的端口 8888：


    ./nc-rust -u 192.168.2.32 -p 8888