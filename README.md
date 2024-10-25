# 1724Project


## Motivation


## Objective and key features

##### 1. User Sign up & Log in {#user-sign-up-log-in}
##### 2. Basic User Authentication
##### 3. Chat room or Channel creation and joining {#Chat-room}
##### 4. Real-time messaging using WebSockets (an industry standard)
##### 5. Presence detection to show online/offline status
##### 6. Persistent Storage of Messages
##### 7. Message History{#Message-History}
##### 8. Frontend Implement using Rust{#Frontend}

## Tentative plan
Briefly and concisely, describe how your team plans to achieve the project objective in a matter of weeks, with clear descriptions of responsibilities for each team member in the team. As the duration of the project is quite short, there is no need to include milestones and tentative dates.

The proposed plan is concise and clear, includes responsibilities for each team member, and a casual reader can be convinced that the project can be reasonably completed by the project due date. (10 Points)

In this project, our objective is to design and implement a robust, scalable chat server application utilizing cutting-edge web technologies based on Rust. The backend architecture will be constructed using the [Rocket](https://rocket.rs/guide/v0.5/upgrading/#blocking-i-o) and [Actix](https://actix.rs/docs/server) frameworks, which offer flexibility and efficiency. For testing the backend requests, [Postman](https://www.postman.com/) will be employed to ensure reliability and accuracy. User data management will be handled by the [SurrealDB](https://surrealdb.com/) database, providing a solid and scalable storage solution. Real-time communication between the server and clients will be enabled through the [WebSocket](https://rocket.rs/guide/v0.5/upgrading/#blocking-i-o) protocol, supporting bidirectional data flow over TCP connections. For front-end development, we will utilize the [Yew](https://yew.rs/docs/getting-started/introduction) framework to create an interactive and responsive user interface for the chat client. Additionally, as project timelines permit, we will enhance the front-end aesthetics to improve user engagement and experience.

### [Build up server and chat functionality](#Chat-room)
The first step of the whole project is to build a server. Setting up HTTP Rocket is initial step which means it creates a basic HTTP server that serves an HTML page for the chat application. We will add routes to serve static files such as the main HTML page, JavaScript, and other assets. In the second step of our chat application development using Actix, we will employ the Actix WebSocket Actor to manage WebSocket connections effectively, facilitating real-time communication by broadcasting messages between clients. Concurrently, we will design a robust Chatroom Actor responsible for orchestrating the dynamics of chatrooms—such as the creation of new rooms and the dissemination of messages to all participants within a chatroom. Initially, our focus will be on establishing a functional one-on-one guest chat in a designated chatroom to validate our implementation. Following the successful demonstration of this feature, we will enhance the system to support multiple users concurrently interacting within the same chatroom, thereby expanding the application's capabilities to accommodate group discussions and interactions seamlessly.

### [User Sign-up and user authentication with Database](#user-sign-up-log-in) 
After successfully establishing the chat functionality, our next objective will be to develop and integrate user registration and authentication processes. This will be achieved using SurrealDB, which will store critical user information, including usernames, passwords, and their online/offline statuses. Each time a user attempts to log in to the chatroom, the server will query this database to retrieve and verify the user's credentials, thereby ensuring secure and authenticated access to the system.


### [Chat Message with Database](#Message-History)
To guarantee the preservation of messages exchanged within the chat application post-session, it is essential to utilize a database for storage. SurrealDB will serve as the repository for all messages, cataloging each with associated metadata such as timestamps, sender IDs, and recipient IDs or room IDs. This database integration not only safeguards the data but also facilitates the retrieval of chat histories. Such functionality is indispensable for providing a continuous and engaging user experience. Upon accessing a chat, users will initially see the most recent messages; as they navigate backward, older messages will be dynamically fetched from the database in an incremental manner. 

### [Display with Frontend](#Frontend)

In the initial phase, we will utilize the command line to thoroughly test the functionality of the backend components. Once the backend is verified, we will proceed to develop a user-friendly interface for the chat application using the Yew framework, a modern and efficient web development framework for Rust. The frontend will be designed with a focus on responsiveness and intuitiveness, ensuring that users can seamlessly interact with the chat application. It will manage WebSocket connections to the backend, facilitating real-time communication between the server and clients. The interface will incorporate essential components such as user authentication, room creation and joining, and real-time message display. Furthermore, the frontend will include functionality to display the online or offline status of users, enriching the overall user experience by providing comprehensive interaction features and enhancing engagement.





There are three students in the team, each with their own responsibilities:





Task 1: Set up the server using Rocket to handle HTTP requests, including user sign-up and login.
Task 2: Integrate Actix for managing WebSocket connections, enabling real-time messaging, and ensuring efficient handling of chatrooms.
Task 3: Implement authentication mechanisms for users, including JWT or cookie-based authentication.
Task 4: Implement presence detection for users (online/offline status).
Task 5: Design and implement message broadcasting between chat rooms.
Task 2: Create APIs to store and retrieve chat messages, ensuring persistent storage of messages.
Task 3: Implement user presence tracking and persistent message storage.
Task 4: Work on message history retrieval so that users can see past messages when joining a chat room.

Task 2: Integrate WebSocket communication with the backend to handle real-time messaging.
Task 3: Implement UI components for user authentication, room creation/joining, and message display.
Task 4: Add support for showing the online/offline status of users.

- Henry:
Task 1: Set up a database (SurrealDB) for storing user credentials, chat room metadata, and message history.
Task 1: Build the Yew frontend for the chat application, creating a responsive interface for user interaction.


- Sheila:
- Selena: