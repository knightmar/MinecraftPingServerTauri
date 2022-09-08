#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::{io::{Write, Read}, net::{TcpStream, Shutdown}};
use std::intrinsics::try;
use std::ptr::null;

use bytebuffer::ByteBuffer;

fn write_var_int_bytebuffer(buffer: &mut ByteBuffer, value: i64) -> std::io::Result<()> {
    let mut var = value;
    loop {
        if (var & 0xFFFFFF80) == 0 {
            buffer.write_all(&[var as u8])?;
            return Ok(());
        }
        buffer.write_all(&[(var & 0x7F | 0x80) as u8])?;
        var >>= 7;
    }
}

fn write_var_int_tcpstream(buffer: &mut TcpStream, value: i64) -> std::io::Result<()> {
    let mut var = value;
    loop {
        if (var & 0xFFFFFF80) == 0 {
            buffer.write_all(&[var as u8])?;
            return Ok(());
        }
        buffer.write_all(&[(var & 0x7F | 0x80) as u8])?;
        var >>= 7;
    }
}

fn read_var_int(stream: &mut TcpStream) -> i64 {
    let mut i = 0;
    let mut j = 0;
    loop {
        let mut buf = [0; 1];
        stream.read_exact(&mut buf).unwrap();
        let k = buf[0] as i64;
        i |= (k & 0x7F) << j * 7;
        j += 1;
        if j > 5 {
            panic!("VarInt too big");
        }
        if (k & 0x80) != 128 {
            break;
        }
    }
    i
}

#[tauri::command]
fn ping(host: &str, port: &str) -> String {
    let mut stream_open = TcpStream::connect(host.to_owned() + ":" + port);
    let mut stream = match stream_open {
        Ok(s) => s,
        Err(_err) => null,
    };


    let mut buffer = ByteBuffer::new();
    buffer.write_all(&[0x00]).expect("TODO: panic message");
    write_var_int_bytebuffer(&mut buffer, 4).expect("TODO: panic message");
    write_var_int_bytebuffer(&mut buffer, host.len() as i64).expect("TODO: panic message");
    buffer.write_all(host.as_bytes()).expect("TODO: panic message");
    buffer.write_all(25565u16.to_be_bytes().as_ref()).expect("TODO: panic message");
    write_var_int_bytebuffer(&mut buffer, 1).expect("TODO: panic message");

    write_var_int_tcpstream(&mut stream, buffer.len() as i64).expect("TODO: panic message");
    stream.write_all(&buffer.to_bytes()).expect("TODO: panic message");
    stream.write_all(&[0x01, 0x00]).expect("TODO: panic message");
    stream.flush().expect("TODO: panic message");
    println!("Sent");
    read_var_int(&mut stream);
    let id = read_var_int(&mut stream);
    if id == -1 {
        return String::from("ErrorPremature end of stream.");
    }
    if id != 0x00 {
        return String::from("ErrorInvalid packet ID.");
    }
    let json_len = read_var_int(&mut stream);
    if json_len == -1 {
        return String::from("ErrorPremature end of stream.");
    }
    if json_len == 0 {
        return String::from("ErrorInvalid string length.");
    }
    let mut input = vec![0; json_len as usize];
    stream.read_exact(&mut input).expect("TODO: panic message");
    let json = String::from_utf8(input).unwrap();

    buffer.clear();
    stream.shutdown(Shutdown::Both).expect("TODO: panic message");
    println!("Server : {} \n\n{}", host, json);

    json
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}