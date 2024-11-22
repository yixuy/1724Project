### For the backend in MacOS
- install before runnng the database
`brew install surrealdb/tap/surreal`
##### Run the database in the terminal 
1. `cd server`
2. `surreal start --log info --user root --pass root --bind 127.0.0.1:5050 file://mydatabase.db`
3. copy the `127.0.0.1:5050` in the browser to test the database

##### Run the backend in the terminal 
- install before runnng the backend
`cargo install cargo-watch`  
1. `cd server`
when you do the server for your sake you can do backend reloading automatically 
1. `cargo watch -x run`
2.use `http://127.0.0.1:5000/test` in the POSTMAN to test the server whether is working or not

### For the frontend in MacOS
- install before runnng the frontend
`rustup target add wasm32-unknown-unknown`
`cargo install --locked trunk`
##### Run the frontend 
1. `cd client`
2. `trunk serve --open` 







