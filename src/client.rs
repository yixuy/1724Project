use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    // 连接 WebSocket 服务器
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080/ws/")
        .await
        .expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();
    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    // Step 1: Ask for the user's name
    println!("Enter your name:");
    let name = match lines.next_line().await {
        Ok(Some(name)) => name,
        Ok(None) => {
            eprintln!("Error reading input.");
            return;
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return;
        }
    };

    // Step 2: Ask whether the user wants to create a new room or join an existing one
    println!("Do you want to create a new chat room or join an existing one?");
    println!("Type 'create' to create a new room or 'join' to join an existing room:");

    let room_action = match lines.next_line().await {
        Ok(Some(action)) => action,
        Ok(None) => {
            eprintln!("Error reading input.");
            return;
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return;
        }
    };

    // Step 3: Handle the room action
    let room_name = if room_action.to_lowercase() == "create" {
        println!("Enter the new room name:");
        match lines.next_line().await {
            Ok(Some(room_name)) => room_name,
            Ok(None) => {
                eprintln!("Error reading input.");
                return;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
    } else if room_action.to_lowercase() == "join" {
        println!("Enter the room name you want to join:");
        match lines.next_line().await {
            Ok(Some(room_name)) => room_name,
            Ok(None) => {
                eprintln!("Error reading input.");
                return;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
    } else {
        eprintln!("Invalid action, must be 'create' or 'join'.");
        return;
    };

    // Step 4: Send room creation/join message to server
    write
        .send(Message::Text(format!("/join {}", room_name)))
        .await
        .unwrap();

    // Step 5: Start listening for messages
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("{}", text);
            }
        }
    });

    // Step 6: Send messages --- with the user's name attached
    println!("You can now start chatting!");

    while let Ok(Some(line)) = lines.next_line().await {
        let message = format!("{}: {}", name, line);
        write.send(Message::Text(message)).await.unwrap();
    }
}