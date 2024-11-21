### For the installation:
##### Go to the director 'Client'
`cd client`
##### Install WebAssembly target
`rustup target add wasm32-unknown-unknown`
##### Install Trunk
`cargo install --locked trunk`
##### Run the frontend 
`trunk serve --open` 


##### For the backend in MacOS
`brew install surrealdb/tap/surreal`
##### Run the database in the terminal 
`surreal start --log info --user root --pass root --bind 127.0.0.1:5050 file://mydatabase.db`

`copy the `127.0.0.1:5050` in the browser to test the database` 

`cargo install cargo-watch` 
when you do the server for your sake you can do backend reloading automatically 
`cargo watch -x run`


use `http://127.0.0.1:5000/test` in the POSTMAN to test the server whether is working or not