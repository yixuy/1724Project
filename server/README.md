### For the installation:
##### Go to the director 'Client'
`cd client`
##### Install WebAssembly target
`rustup target add wasm32-unknown-unknown`
##### Install Trunk
`cargo install --locked trunk`
##### Run the frontend 
`trunk serve --open` 


##### For the backend For MacOS
`brew install surrealdb/tap/surreal`
##### Run the database in the terminal 
`surreal start --log info --user root --pass root --bind 127.0.0.1:8001 file://mydatabase.db`

`copy the `127.0.0.1:8001` in the browser to test the database` 

`cargo install cargo-watch` when you do the server for your sake you can do backend reloading automatically 
`cargo watch -x run`