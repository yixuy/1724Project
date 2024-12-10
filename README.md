# ECE1724 Course Project Proposal

| Student Name|  Student ID |  
|-------------|-------------|
| Zhaoyu (Selena) Yan  | 1006521621 |
| Yiran (Sheila) Chen | 1004392739 |
| Yixu (Henry) Ye  | 1010506579 |

## Motivation

Real-time communication is central to our digital lives, shaping everything from work meetings to casual catch-ups with friends. While many established chat applications exist, they often face performance and scalability challenges under high user loads. Several of these platforms may struggle to provide low-latency communication and a robust infrastructure for thousands of concurrent users.

Motivated by these common issues, our team has decided to address them in our project by leveraging the unique strengths of Rust. While domains like web and network application development are typically dominated by languages like JavaScript and Python—each with mature ecosystems and extensive tooling—Rust offers standout features that make it an ideal candidate for building real-time systems such as chat applications. The potential advantages of Rust include its memory safety guarantees, concurrency model, and performance, which are crucial for building scalable and efficient real-time communication systems. Rust’s memory safety without garbage collection ensures that we can achieve predictable performance without the overhead of runtime memory management, which is crucial for latency-sensitive applications. Its ownership and borrowing model allows for safe concurrency, preventing common multithreading errors like data races. Rust’s message-passing capabilities and asynchronous programming model allow us to efficiently handle large numbers of simultaneous connections, making it a great fit for network-heavy applications. Besides, its strong type system helps us catch bugs at compile time, significantly reducing runtime errors, which enhances both reliability and maintainability. We are driven by the potential to enhance user experience through these unique advantages, ensuring that our application can handle high traffic without compromising performance.

Although Rust has been gaining popularity, it is still less commonly used for building mainstream chat applications, leaving a gap in the market for a Rust-native solution. We aim to fill this gap by developing a real-time chat application written entirely in Rust for our course project. This project will not only enable us to solve real-world problems but also showcase Rust's potential in creating high-level, user-facing applications beyond its typical system programming uses.

## Objective and Key Features

The objective of this project is to design and implement a high-performance real-time chat application in Rust, enabling users to register, log in, create or join chat rooms, and exchange messages instantly. This application will serve as a demonstration of Rust's capabilities in building a reliable and efficient communication system, with a strong emphasis on enhancing backend performance while ensuring frontend usability.

Considering that most chat applications built with Rust currently rely on frontend frameworks like React, and there has been limited exploration into using Rust-related frameworks for frontend development. This project aims to leverage the Yew framework to create the user interface of the chat application, thereby, filling a gap in the current Rust ecosystem. The backend will be developed using Actix-Web. Ultimately, the goal is to create a fully functional chat application using nearly 100% Rust.

### Key Features

This project will focus on implementing several key features to ensure a seamless user experience. Here are the key features of the chat application:

#### 1. User registration

New users could be able to create unique accounts through a registration process. User credentials will be securely stored in a database.

#### 2. User login and Basic user authentication

The application will include an authentication mechanism to manage user access. Each user will be required to sign up and log in before they can create or join chat rooms. Once users complete the registration process, they can log in using their own usernames and passwords. Authentication ensures that only authorized users can access the chat application and its features, promoting safe interactions.

#### 3. Chat room creation and joining

Users will be able to create new chat rooms with room numbers, and other users can join these rooms to engage in conversations. Each chat room will be isolated, meaning that messages sent in one room will not be visible to users in other rooms. This feature enables users to engage in separate discussions within different chat rooms, enhancing privacy.

#### 4. Real-time messaging using WebSockets

The application will utilize WebSocket technology to enable real-time, two-way communication between clients and server. This feature allows messages to be transmitted immediately, enhancing user experience.

#### 5. User Online/Offline status detection

The system will include User Online/Offline status detection to show whether a user is online or offline in a chat room. This functionality will improve user engagement by enabling participants to see who is currently active in the conversation.

#### 6. Message persistence and history retrieval

To ensure a reliable communication experience, messages exchanged in chat rooms will be stored in a database, providing both persistence and accessible message history. This feature prevents any message loss due to unexpected disconnections or browser refreshing. Users can retrieve previous messages upon re-entering a chat room, as the stored messages will be reloaded from the database, allowing them to seamlessly continue their conversations from where they left off.

#### 7. Frontend user interface

A user-friendly interface will be created, enabling users to register, log in, create or join chat rooms, and exchange messages in real time. The design will focus on simplicity and ease of use, ensuring a smooth experience for all users.



