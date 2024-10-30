# ECE1724 Course Project Proposal

| Student Name|  Student ID |  
|-------------|-------------|
| Zhaoyu (Selena) Yan  | 1006521621 |
| Yiran (Sheila) Chen | 1004392739 |
| Yixu (Henry) Ye  | 1010506579 |

## Motivation

Real-time communication is central to our digital lives, shaping everything from work meetings to casual catch-ups with friends. While many established chat applications exist, they often face performance and scalability challenges under high user loads. Several of these platforms may struggle to provide low-latency communication and a robust infrastructure for thousands of concurrent users.

Motivated by these common issues, our team has decided to address them in our project by leveraging the unique strengths of Rust. While domains like web and network application development are typically dominated by languages like JavaScript and Python—each with mature ecosystems and extensive tooling—Rust offers standout features that make it an ideal candidate for building real-time systems such as chat applications. The potential advantages of Rust include its memory safety guarantees, concurrency model, and performance, which are crucial for building scalable and efficient real-time communication systems. Rust’s memory safety without garbage collection ensures that we can achieve predictable performance without the overhead of runtime memory management, which is crucial for latency-sensitive applications. Its ownership and borrowing model allows for safe concurrency, preventing common multithreading errors like data races. Rust’s message-passing capabilities and asynchronous programming model allow us to efficiently handle large numbers of simultaneous connections, making it a great fit for network-heavy applications. Besides, its strong type system helps us catch bugs at compile time, significantly reducing runtime errors, which enhances both reliability and maintainability. We are driven by the potential to enhance user experience through these unique advantages, ensuring that our application can handle high traffic without compromising performance.

Although Rust has been gaining popularity, it is still less commonly used for building mainstream chat applications, leaving a gap in the market for a Rust-native solution. We aim to fill this gap by developing a real-time chat application entirely written in nearly 100% Rust for our course project. This project will not only enable us to solve real-world problems but also showcase Rust's potential in creating high-level, user-facing applications beyond its typical system programming uses.

## Objective and key features

The objective of this project is to design and implement a high-performance real-time chat application in Rust, enabling users to register, log in, create or join chat rooms, and exchange messages instantly. This application will serve as a demonstration of Rust's capabilities in building a reliable and efficient communication system, with a strong emphasis on enhancing backend performance while ensuring frontend usability.

Considering that most chat applications built with Rust currently rely on front-end frameworks like React, and there has been limited exploration into using Rust-related frameworks for front-end development. This project aims to leverage the Yew framework to create the user interface of the chat application, thereby, filling a gap in the current Rust ecosystem. The backend will be developed using Actix-Web. Ultimately, the goal is to create a fully functional chat application using nearly 100% Rust.

This project will focus on implementing several key features to ensure a seamless user experience. Here are the key features of the chat application:

### 1. User registration

New users could be able to create unique accounts through a registration process. User credentials will be securely stored in a database.

### 2. User login and Basic user authentication

The application will include an authentication mechanism to manage user access. Each user will be required to sign up and log in before they can create or join chat rooms. Once users complete the registration process, they can log in using their own usernames and passwords. Authentication ensures that only authorized users can access the chat application and its features, promoting safe interactions.

### 3. Chat room creation and joining

Users will be able to create new chat rooms with room numbers, and other users can join these rooms to engage in conversations. Each chat room will be isolated, meaning that messages sent in one room will not be visible to users in other rooms. This feature enables users to engage in separate discussions within different chat rooms, enhancing privacy.

### 4. Real-time messaging using WebSockets

The application will utilize WebSocket technology to enable real-time, two-way communication between clients and server. This feature allows messages to be transmitted immediately, enhancing user experience.

### 5. User Online/Offline status detection

The system will include User Online/Offline status detection to show whether a user is online or offline in a chat room. This functionality will improve user engagement by enabling participants to see who is currently active in the conversation.

<!-- ### 6. Message persistence

Messages exchanged in chat rooms will be stored in a database to ensure persistence. This feature prevents any loss of messages due to refresh the browser or unexpected disconnections, providing a more reliable communication experience. -->

### 6. Message persistence and history retrieval

To ensure a reliable communication experience, messages exchanged in chat rooms will be stored in a database, providing both persistence and accessible message history. This feature prevents any message loss due to unexpected disconnections or application shutdowns. Users can retrieve previous messages upon re-entering a chat room, as the stored messages will be reloaded from the database, allowing them to seamlessly continue their conversations from where they left off.

### 7. Front-end user interface

A user-friendly interface will be created, enabling users to register, log in, create or join chat rooms, and exchange messages in real time. The design will focus on simplicity and ease of use, ensuring a smooth experience for all users.

### 8. Creating all the API connected the frontend and backend

The application will include a comprehensive set of RESTful APIs to facilitate communication between the front-end and back-end. These robust APIs will enable the front-end to interact with the back-end, allowing users to perform various actions such as creating or joining chat rooms, sending and receiving messages, and managing user accounts.

### Group Member Work Allocation

**Selena:**

- User Registration
- User Login and Basic User Authentication  
- User Online/Offline status detection

**Sheila:**

- Chat Room Creation and Joining
- Real-Time Messaging Using WebSockets  
- Message persistence and history retrieval

**Henry:**

- Build up the Database and Integration
- Front-End User Interface  
- Creating all the API connected the frontend and backend

## Tentative plan

In this project, our objective is to design and implement a robust, scalable chat server application utilizing cutting-edge web technologies based on Rust. The backend architecture will be constructed using [Actix-Web](https://actix.rs/docs/server) frameworks, which offer flexibility and efficiency. For testing the backend requests, [Postman](https://www.postman.com/) will be employed to ensure reliability and accuracy. User data management will be handled by the [SurrealDB](https://surrealdb.com/) database, providing a solid and scalable storage solution. Real-time communication between the server and clients will be enabled through the [WebSocket](https://rocket.rs/guide/v0.5/upgrading/#blocking-i-o) protocol, supporting bidirectional data flow over TCP connections. For front-end development, we will utilize the [Yew](https://yew.rs/docs/getting-started/introduction) framework (Since we are using Yew, the HTML will be dynamically generated, with the majority of the code written in Rust. While most of the functionality will be implemented in Rust, we may also incorporate existing JavaScript libraries where necessary to enhance or extend specific features) to create an interactive and responsive user interface for the chat client and use Actix-Web to build the REST API in order to make the frontend and backend communicate with each other smoothly. Additionally, as project timelines permit, we will enhance the front-end aesthetics to improve user engagement and experience. Based on the allocation of responsibilities, Selena will oversee user authentication, including login and registration processes, as well as manage user presence within the chat room. Sheila will be responsible for backend development, which entails setting up the server and handling WebSocket communication and the message history. At the same time, Henry will focus on front-end development, which includes designing the user interface, managing user interactions, create and test API using and integrating the database. These tasks will be implemented in a structured, step-by-step manner to ensure smooth development progress.

### Build up server and chat functionality
<!-- (#Chat-room) -->

The first phase of the project focuses on building the server. We will use the command line interface to display and test the server’s functionality, as well as the basic chat features. During the development of our chat application using Actix-Web, Sheila will utilize the Actix-Web WebSocket Actor to effectively manage WebSocket connections. This will enable real-time communication by broadcasting messages between clients, ensuring smooth interactions within chatrooms. The integration of Actix-Web for WebSocket management will enhance the overall efficiency of message handling and communication within the system. Additionally, WebSocket communication will be integrated with the backend to ensure real-time messaging and synchronization across all connected clients. At the same time, Sheila will design and implement message broadcasting between chat rooms, as well as create a robust Chatroom Actor responsible for managing chatroom dynamics. This includes tasks such as creating new rooms and disseminating messages to all participants within each chatroom. Initially, we will focus on establishing a functional one-on-one chat within a designated chatroom to validate our implementation. Upon successful completion of this feature, we will extend the system to support multiple users interacting concurrently in the same chatroom, expanding the application’s capabilities to facilitate group discussions and seamless multi-user interactions.

### User Sign-up and user authentication with Database
<!-- (#user-sign-up-log-in)  -->
After successfully implementing the chat functionality, our next objective is to develop and integrate user registration and authentication. We will use SurrealDB to store essential user information, including usernames, hashed passwords, and online/offline statuses. Henry will be responsible for defining the data structure within SurrealDB, while Selena will configure the server using Actix-Web to manage user sign-up and login processes. When a user attempts to log in, the server will query the database to securely retrieve and verify their credentials, ensuring authenticated access to the chatroom. Selena will also implement robust authentication mechanisms, such as JWT (JSON Web Tokens) or cookie-based authentication, to maintain user sessions and enhance security. Additionally, she will integrate functionality to display users' online/offline statuses, further enriching the user experience and interaction within the application.

### Chat Message with Database
<!-- (#Message-History) -->
To ensure the preservation of chat messages after each session, integrating a database is crucial. The reason why we work on message history retrieval is mainly because users can see past messages when joining a chat room. Henry will set up SurrealDB to store user credentials, chat room metadata, and message history, while Sheila will develop APIs for saving and retrieving chat messages, ensuring persistent storage. SurrealDB will act as the central repository for all messages, cataloging them with relevant metadata such as timestamps, sender IDs, and recipient IDs or room IDs. This integration not only secures data but also allows users to retrieve their chat histories. Such functionality is indispensable for providing a continuous and engaging user experience. Upon accessing a chat, users will initially see the most recent messages; as they navigate backward, older messages will be dynamically fetched from the database in an incremental manner.

### Display with Frontend
<!-- (#Frontend) -->

Once the backend has been validated feature by feature, we will shift our focus to developing a user-friendly interface for the chat application using the Yew framework, a modern and efficient web development tool for Rust. Henry will take charge of building the Yew frontend, creating a responsive and intuitive interface that incorporates key features such as user authentication, room creation and joining, and message display. Henry will also ensure seamless integration between the frontend and backend by implementing the necessary RESTful API endpoints using Actix-Web. The frontend will be designed to work in harmony with these APIs, allowing both to be developed in parallel. Special attention will be given to making the interface highly responsive and easy to use, ensuring a smooth and engaging user experience. The frontend will also manage WebSocket connections to enable real-time communication between the server and clients. Additionally, the interface will include features like displaying user statuses (online/offline) to enhance engagement and interactivity. Once the frontend and backend are fully integrated, we will ensure the entire chat application is functional and prepared for testing and deployment.
