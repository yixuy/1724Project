use tokio::io::{self, AsyncBufReadExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    
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
        _ => {
            eprintln!("Failed to read your name.");
            return;
        }
    };

    // update''
    //let mut current_room: Option<String> = None;  // Track the current room

    // Step 2: Ask for create or join
    let room_action = loop {
        println!("Do you want to create a new chat room or join an existing one?");
        println!("Type 'create' to create a new room or 'join' to join an existing room:");
        match lines.next_line().await {
            Ok(Some(action)) if action.to_lowercase() == "create" || action.to_lowercase() == "join" => break action,
            Ok(Some(_)) => println!("Invalid action. Please type 'create' or 'join'."),
            _ => {
                eprintln!("Failed to read your action.");
                return;
            }
        }
    };

    // Step 3: Ask for room number
    let room_name = match room_action.to_lowercase().as_str() {
        "create" => {
            println!("Enter the new room name:");
            match lines.next_line().await {
                Ok(Some(name)) => name,
                _ => {
                    eprintln!("Failed to read the room name.");
                    return;
                }
            }
        },
        "join" => {
            println!("Enter the room name you want to join:");
            match lines.next_line().await {
                Ok(Some(name)) => name,
                _ => {
                    eprintln!("Failed to read the room name.");
                    return;
                }
            }
        },
        _ => {
            eprintln!("Invalid action.");
            return;
        }
    };

    // Step 4: Send corresponding command to the server
    let command = format!("/{action} {room}", action = room_action, room = room_name);
    write.send(Message::Text(command)).await.unwrap();

    // Step 5: Handle server response
    let server_response = match read.next().await {
        Some(Ok(Message::Text(text))) => text,
        _ => {
            eprintln!("Failed to receive server response.");
            return;
        }
    };

    // Step 6: Interpret server response
    if server_response.contains("already exists") {
        println!("{}", server_response);
        println!("Would you like to join the existing room? (yes/no)");
        let join_existing = match lines.next_line().await {
            Ok(Some(answer)) if answer.to_lowercase() == "yes" => true,
            _ => false,
        };

        if join_existing {
            write
                .send(Message::Text(format!("/join {}", room_name)))
                .await
                .unwrap();
            //current_room = Some(room_name.clone()); 
        } else {
            println!("Exiting...");
            return;
        }
    } else if server_response.contains("created successfully") {
        println!("{}", server_response);
        write
            .send(Message::Text(format!("/join {}", room_name)))
            .await
            .unwrap();
        //current_room = Some(room_name.clone()); 
    } else {
        println!("{}", server_response); 
    }

    // Step 7: Start listening for messages
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(text) = msg {
                println!("{}", text);
            }
        }
    });

    /*
    while let Ok(Some(line)) = lines.next_line().await {
        let message = format!("{}: {}", name, line);

        write.send(Message::Text(message)).await.unwrap();
    }
    */

    loop {
        let message = match lines.next_line().await {
            Ok(Some(line)) => line,
            _ => break,
        };

        if message.to_lowercase() == "/exit" {
            println!("Exiting chat...");
            write.send(Message::Text("/exit".to_string())).await.unwrap();
            break;
        } 
        else {
            let formatted_message = format!("{}: {}", name, message);
            write.send(Message::Text(formatted_message)).await.unwrap();
        }
    }

    // Close the connection when exiting
    write.send(Message::Close(None)).await.unwrap();

}

