use async_chat::{
    utils::{self, ChatResult},
    FromClient, FromServer,
};
use async_std::{io, net, prelude::*, task};

fn parse_command(command: &str) -> Option<FromClient> {
    let words: Vec<&str> = command
        .trim()
        .splitn(3, ' ')
        .filter(|word| !word.is_empty())
        .collect();
    if words.len() < 2 {
        return None;
    }
    match words[0] {
        "join" if words.len() == 2 => Some(FromClient::Join {
            group_name: words[1].to_owned().into(),
        }),
        "post" if words.len() == 3 => Some(FromClient::Post {
            group_name: words[1].to_owned().into(),
            message: words[2].to_owned().into(),
        }),
        _ => None,
    }
}

fn split_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    match input.find(char::is_whitespace) {
        Some(i) => Some((&input[..i], &input[i..])),
        None => Some((input, "")),
    }
}

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "\
    Commands:\n\
    join GROUP\n\
    post GROUP MESSAGE\n\
    Type Control-D (on Unix) or Control-Z (on Windows) to close the connection."
    );

    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_line) = command_lines.next().await {
        let command = command_line?;
        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }
    Ok(())
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("message posted to {}: {}", group_name, message);
            }
            FromServer::Error(message) => {
                println!("error from server: {message}");
            }
        }
    }
    Ok(())
}

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;
        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);
        from_server.race(to_server).await?;
        Ok(())
    })
}
