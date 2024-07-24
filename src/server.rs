use std::{fs::{remove_file, set_permissions, OpenOptions, Permissions}, os::unix::fs::PermissionsExt, process::{self, Stdio}};
use nix::libc::kill;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{UnixListener, UnixStream}, process::{Child, Command}, select, signal::{self, unix::SignalKind}};

use crate::{shell_path, socket_path, PROCESS_EXIT, PROCESS_EXIT_BY_SIGNAL};

struct ChildWrapper(Child);

impl Drop for ChildWrapper {
    fn drop(&mut self) {
        let child=&mut self.0;
        child.start_kill().ok();
    }
}

fn get_option()->OpenOptions{
    let mut option= OpenOptions::new();
    option.read(true).write(true);
    option
}

async fn handle_client(mut stream : UnixStream){
    //open shell and forward IO
    let mut buf:[u8;8]=[0;8];
    let uid=match stream.read(&mut buf).await{
        Ok(size)=>{
            if size!=4{
                eprintln!("wrong uid size");
                return;
            }
            let uid_buf:[u8;4]=buf[0..4].try_into().unwrap();
            u32::from_be_bytes(uid_buf)
        }
        _=>{
            eprintln!("read error");
            return;
        }
    };
    println!("receive uid {} request",uid);
    let stdin=match get_option().open(format!("/proc/{}/fd/0",uid)){
        Ok(file)=>{file}
        _=>{
            eprintln!("failed to open stdin");
            return;
        }
    };
    let stdout=match get_option().open(format!("/proc/{}/fd/1",uid)){
        Ok(file)=>{file}
        _=>{
            eprintln!("failed to open stdout");
            return;
        }
    };
    let stderr=match get_option().open(format!("/proc/{}/fd/2",uid)){
        Ok(file)=>{file}
        _=>{
            eprintln!("failed to open stderr");
            return;
        }
    };

    let mut command=Command::new(shell_path!());
    command.stdin(Stdio::from(stdin))
    .stdout(Stdio::from(stdout))
    .stderr(Stdio::from(stderr));

    let child=match command
    .spawn(){
        Ok(child)=>{child}
        _=>{return;}
    };
    let child_pid=match child.id(){
        Some(id)=>{id}
        None=>{
            eprintln!("failed to get child pid");
            return;
        }
    } as i32;
    println!("child {} started",child_pid);
    match stream.write(child_pid.to_be_bytes().as_slice()).await{
        Ok(_)=>{},
        _=>{
            return;
        }
    }
    let mut child_wrapper=ChildWrapper(child);
    
    loop{
        select! {
            read_res = stream.read(&mut buf)=>{
                match read_res{
                    Ok(size)=>{
                        if size!=4{
                            eprintln!("bad signal size");
                            break;
                        }
                        let signal_buf:[u8;4]=buf[0..4].try_into().unwrap();
                        let signal=u32::from_be_bytes(signal_buf);
                        println!("send signal {}",signal);
                        unsafe{
                            kill(child_pid, signal as i32);
                        }
                    }
                    _=>{
                        eprintln!("read signal fail");
                        break;
                    }
                }
            }
            Ok(code) = child_wrapper.0.wait()=>{
                match code.code(){
                    Some(code)=>{
                        println!("process exit with {}",code);
                        buf[0]=PROCESS_EXIT;
                        buf[1..5].copy_from_slice(code.to_be_bytes().as_slice());
                        match stream.write(&buf[0..5]).await{
                            Ok(_)=>{
                            }
                            _=>{
                                eprintln!("failed to write exit code")
                            }                    
                        }
                        return;
                    }
                    None=>{
                        println!("process exit by signal");
                        match stream.write(&[PROCESS_EXIT_BY_SIGNAL]).await{
                            Ok(_)=>{
                            }
                            _=>{
                                eprintln!("failed to write exit code")
                            }                    
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub async fn run_server() {
    let uid=nix::unistd::getuid().as_raw();
    if uid!=0{
        eprintln!("please run daemon as root");
        process::exit(-1);
    }
    let socket_path = socket_path!();
    remove_file(socket_path).ok();
    let listener = UnixListener::bind(socket_path).unwrap();
    let permissions=Permissions::from_mode(0o666);
    set_permissions(socket_path, permissions).unwrap();

    let mut sigint = signal::unix::signal(SignalKind::interrupt()).unwrap();

    loop{
        select! {
            accept_res = listener.accept() =>{
                match accept_res {
                    Ok((socket,_))=>{
                        tokio::spawn(handle_client(socket));
                    }
                    Err(err)=>{
                        eprintln!("accept error: {}",err);
                        process::exit(-2);
                    }
                }
            }
            _ = sigint.recv() =>{
                break;
            }
        }
    }
}
