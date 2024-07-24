use std::process;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::UnixSocket, select, signal::unix::{signal, SignalKind}};

use crate::{socket_path, PROCESS_EXIT_BY_SIGNAL};
use crate::PROCESS_EXIT;


pub async fn run_client() {
    let conn=UnixSocket::new_stream().unwrap();
    let mut stream=conn.connect(socket_path!()).await.unwrap();
    
    //write uid
    let uid_buf=process::id().to_be_bytes();
    stream.write(&uid_buf).await.unwrap();

    let mut buf:[u8;8]=[0;8];

    //wait pid
    match stream.read(&mut buf).await{
        Ok(size)=>{
            if size!=4 {
                process::exit(-9)
            }
        }
        _=>{
            process::exit(-10)
        }
    }

    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    loop{
        select! {
            //读取退出值
            read_res = stream.read(&mut buf)=>{
                match read_res{
                    Ok(_)=>{
                        match buf[0]{
                            PROCESS_EXIT=>{
                                let code_buf:[u8;4]=buf[1..5].try_into().unwrap();
                                let code=i32::from_be_bytes(code_buf);
                                process::exit(code);
                            }
                            PROCESS_EXIT_BY_SIGNAL=>{
                                println!("exit by signal");
                                process::exit(-11);
                            }
                            _=>{}
                        };
                    }
                    _=>{
                        process::exit(-12);
                    }
                }
            }
            //读取信号
            _ = sigint.recv() =>{
                let signal_buf=SignalKind::interrupt().as_raw_value().to_be_bytes();
                match stream.write(&signal_buf).await {
                    Ok(_)=>{}
                    _=>{
                        process::exit(-13);
                    }
                }
            }
        }
    }
}
