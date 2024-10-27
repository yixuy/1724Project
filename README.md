# 1724Project


## Motivation
Real-time communication is central to our digital lives, shaping everything from our work meetings to casual catch-ups with friends. While there are many established chat applications, they can sometimes face performance and scalability challenges under high user loads. Quite a few of these platforms may struggle to provide low-latency communication and robust infrastructure for thousands of concurrent users.

Motivated by these common issues, our team chose to address them through our project. We saw an opportunity to utilize the capabilities of Rust in a domain typically dominated by other programming languages like JavaScript and Python, which have mature ecosystems with extensive support for developing web and network applications. However, Rust's standout features, such as memory safety, zero-cost abstractions, and lack of garbage collection, make it ideal for building real-time systems that demand high speed and safety. We are driven by the potential to enhance user experience through these unique advantages, ensuring that our application can handle high traffic without compromising speed.

Although Rust has been gaining popularity, it is still less commonly used for building mainstream chat applications, leaving a gap in the market for a Rust-native solution. We intend to fill this gap by developing a real-time chat application completely written in Rust for our course project. This project will not only allow us to solve real-world problems but also showcase Rust's potential in creating high-level, user-facing applications beyond its typical system programming uses.

## Objective and key features
The objective of this project is to design and implement a high-performance real-time chat application in Rust, enabling users to register, log in, create or join chat rooms, and exchange messages instantly. This application will serve as a demonstration of Rust's capabilities in building a reliable and efficient communication system, with a strong emphasis on enhancing backend performance while ensuring frontend usability.

Considering that most chat applications built with Rust currently rely on front-end frameworks like React, and there has been limited exploration into using Rust-related frameworks for front-end development. This project aims to leverage the Yew framework to create the user interface of the chat application, thereby, filling a gap in the current Rust ecosystem. The backend will be developed using Actix Web. Ultimately, the goal is to create a fully functional chat application using 100% Rust.

This project will focus on implementing several key features to ensure a seamless user experience. Here are the key features of the chat application:

#### 1. User registration
New users could be able to create unique accounts through a registration process. User credentials will be securely stored in a database.

#### 2. User login and Basic user authentication
The application will include an authentication mechanism to manage user access. Each user will be required to sign up and log in before they can create or join chat rooms. Once users complete the registration process, they can log in using their own usernames and passwords. Authentication ensures that only authorized users can access the chat application and its features, promoting safe interactions.

#### 3. Chat room creation and joining
Users will be able to create new chat rooms, and other users can join these rooms to engage in conversations. Each chat room will be isolated, meaning that messages sent in one room will not be visible to users in other rooms. This feature will enable users to engage in different discussions across different chat rooms.

#### 4. Real-time messaging using WebSockets
The application will utilize WebSocket technology to enable real-time, two-way communication between clients and server. This feature allows messages to be transmitted immediately, enhancing user experience.

#### 5. Presence detection
The system will include presence detection to show whether a user is online or offline in a chat room. This functionality will improve user engagement by enabling participants to see who is currently active in the conversation.

#### 6. Message persistence
Messages exchanged in chat rooms will be stored in a database to ensure persistence, preventing any loss of messages due to unexpected disconnections or application shutdowns. 

#### 7. Message History
The chat application will support the retrieval of historical messages in each chat room. This means that if users exit the application, they will still be able to view previous messages upon their next login, as the messages will be reloaded from the database.

#### 8. Front-end user interface
A user-friendly interface will be created, enabling users to register, log in, create or join chat rooms, and exchange messages in real time. The design will focus on simplicity and ease of use, ensuring a smooth experience for all users. 

## Tentative plan

In this project, our objective is to design and implement a robust, scalable chat server application utilizing cutting-edge web technologies based on Rust. The backend architecture will be constructed using [Actix](https://actix.rs/docs/server) frameworks, which offer flexibility and efficiency. For testing the backend requests, [Postman](https://www.postman.com/) will be employed to ensure reliability and accuracy. User data management will be handled by the [SurrealDB](https://surrealdb.com/) database, providing a solid and scalable storage solution. Real-time communication between the server and clients will be enabled through the [WebSocket](https://rocket.rs/guide/v0.5/upgrading/#blocking-i-o) protocol, supporting bidirectional data flow over TCP connections. For front-end development, we will utilize the [Yew](https://yew.rs/docs/getting-started/introduction) framework to create an interactive and responsive user interface for the chat client. Additionally, as project timelines permit, we will enhance the front-end aesthetics to improve user engagement and experience. There are three students in our group, Henry, Selena, and Sheila. Sheila will be responsible for the backend development, including the server setup, WebSocket communication. Henry will focus on the front-end development, creating the user interface and handling user interactions and database integration. Selena will be involved in the user login and user registration process, and the status of user presence in the chat room.

### Build up server and chat functionality
<!-- (#Chat-room) -->

The first step of the whole project is to build a server. Setting up HTTP Rocket is initial step which means it creates a basic HTTP server that serves an HTML page for the chat application. We will add routes to serve static files such as the main HTML page, JavaScript, and other assets. In the second step of our chat application development using Actix, Sheila will utilize the Actix WebSocket Actor to efficiently manage WebSocket connections. This will enable real-time communication by broadcasting messages between clients, ensuring smooth interaction within chatrooms. The integration of Actix for WebSocket management will support message handling and enhance the overall efficiency of the chat system. Additionally, the WebSocket communication will be integrated with the backend to enable real-time messaging and ensure proper synchronization across all clients. Concurrently, Sheila will design and implement message broadcasting between chat rooms and will design a robust Chatroom Actor responsible for orchestrating the dynamics of chatrooms—such as the creation of new rooms and the dissemination of messages to all participants within a chatroom. Initially, our focus will be on establishing a functional one-on-one guest chat in a designated chatroom to validate our implementation. Following the successful demonstration of this feature, we will enhance the system to support multiple users concurrently interacting within the same chatroom, thereby expanding the application's capabilities to accommodate group discussions and interactions seamlessly.

### User Sign-up and user authentication with Database
<!-- (#user-sign-up-log-in)  -->
After successfully establishing the chat functionality, our next objective is to develop and integrate user registration and authentication processes. We will utilize SurrealDB to store key user information such as usernames, hashed passwords, and online/offline statuses. Henry will be responsible for defining the data structure in SurrealDB, while Selena will set up the server using Actix to handle HTTP requests for user sign-up and login. When a user attempts to log in to the chatroom, the server will query the database to retrieve and verify their credentials, ensuring secure and authenticated access. Selena will also implement robust authentication mechanisms, either using JWT (JSON Web Tokens) or cookie-based authentication, to maintain user sessions and enhance security and add support for showing the online/offline status of users.

### Chat Message with Database
<!-- (#Message-History) -->
To ensure the preservation of chat messages after each session, integrating a database is crucial. The reason why we work on message history retrieval is mainly because users can see past messages when joining a chat room. Henry will set up SurrealDB to store user credentials, chat room metadata, and message history, while Sheila will develop APIs for saving and retrieving chat messages, ensuring persistent storage. SurrealDB will act as the central repository for all messages, cataloging them with relevant metadata such as timestamps, sender IDs, and recipient IDs or room IDs. This integration not only secures data but also allows users to retrieve their chat histories. Such functionality is indispensable for providing a continuous and engaging user experience. Upon accessing a chat, users will initially see the most recent messages; as they navigate backward, older messages will be dynamically fetched from the database in an incremental manner.

### Display with Frontend
<!-- (#Frontend) -->

In the initial phase, we will utilize the command line to thoroughly test the functionality of the backend components. Once the backend is verified, we will proceed to develop a user-friendly interface for the chat application using the Yew framework, a modern and efficient web development framework for Rust. The task Henry will do is to build the Yew frontend for the chat application, creating a responsive interface for user interaction as well as implement UI components for user authentication, room creation/joining, and message display. The frontend will be designed with a focus on responsiveness and intuitiveness, ensuring that users can interact with the chat application. It will manage WebSocket connections to the backend, facilitating real-time communication between the server and clients. The interface will incorporate essential components such as user authentication, room creation and joining, and real-time message display. Furthermore, the frontend will include functionality to display the online or offline status of users, enriching the overall user experience by providing comprehensive interaction features and enhancing engagement.
