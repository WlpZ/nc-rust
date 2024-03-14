
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;
use clap::{App, Arg};

// 处理客户端连接的函数
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024]; // 缓冲区，用于读取客户端发送的数据

    loop {
        let bytes_read = stream.read(&mut buffer)?; // 读取客户端发送的数据
        if bytes_read == 0 {
            break; // 如果没有数据可读，则退出循环
        }

        // 执行客户端发送的命令，并将输出发送回客户端
        let command = String::from_utf8_lossy(&buffer[..bytes_read]);
        let output = match Command::new("sh")
            .arg("-c")
            .arg(&command.to_string())
            .output() {
                Ok(output) => output,
                Err(_) => continue, // 如果执行命令失败，则继续循环
        };

        // 将命令执行结果发送回客户端
        stream.write_all(&output.stdout)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Simple Netcat")
        .version("1.0")
        .author("Your Name")
        .about("A simple netcat-like tool")
        .arg(Arg::with_name("local-port")
            .short('l')
            .long("local-port")
            .value_name("LOCAL_PORT")
            .help("Sets the local port to listen on (server mode)")
            .takes_value(true))
        .arg(Arg::with_name("remote-host")
            .short('u')
            .long("remote-host")
            .value_name("REMOTE_HOST")
            .help("Sets the remote host to connect to (client mode)")
            .takes_value(true))
        .arg(Arg::with_name("remote-port")
            .short('p')
            .long("remote-port")
            .value_name("REMOTE_PORT")
            .help("Sets the remote port to connect to (client mode)")
            .takes_value(true))
        .get_matches();

    if let Some(local_port_str) = matches.value_of("local-port") {
        // 作为服务器模式运行
        let local_port: u16 = local_port_str.parse().expect("Invalid local port");
        let listener = TcpListener::bind(format!("0.0.0.0:{}", local_port))?;
        println!("Listening on port {}", local_port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // 处理客户端连接
                    thread::spawn(move || {
                        if let Err(e) = handle_client(stream) {
                            eprintln!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Error accepting connection: {}", e),
            }
        }
    } else if let (Some(remote_host), Some(remote_port_str)) = (matches.value_of("remote-host"), matches.value_of("remote-port")) {
        // 作为客户端模式运行
        let remote_port: u16 = remote_port_str.parse().expect("Invalid remote port");
        let remote_address = format!("{}:{}", remote_host, remote_port);
        let mut stream = TcpStream::connect(remote_address)?;

        // 从标准输入读取数据，并将其发送到服务器
        let mut input = String::new();
        while io::stdin().read_line(&mut input)? > 0 {
            stream.write_all(input.as_bytes())?;
            input.clear();

            // 从服务器接收数据，并将其打印到标准输出
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read])?;
        }
    } else {
        eprintln!("Please provide -l option with local port or -u and -p options with remote host and port");
    }

    Ok(())
}
