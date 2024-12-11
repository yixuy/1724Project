# ECE1724 Course Project - Final Report

| Student Name        | Student ID | Student Email               |
|---------------------|------------|-----------------------------|
| Zhaoyu (Selena) Yan | 1006521621 | zhaoyu.yan@mail.utoronto.ca |
| Yiran (Sheila) Chen | 1004392739 |                             |
| Yixu (Henry) Ye     | 1010506579 |                             |

## Motivation

Real-time communication is central to our digital lives, shaping everything from work meetings to casual catch-ups with friends. While many established chat applications exist, they often face performance and scalability challenges under high user loads. Several of these platforms may struggle to provide low-latency communication and a robust infrastructure for thousands of concurrent users.

Motivated by these common issues, our team has decided to address them in our project by leveraging the unique strengths of Rust. While domains like web and network application development are typically dominated by languages like JavaScript and Python—each with mature ecosystems and extensive tooling—Rust offers standout features that make it an ideal candidate for building real-time systems such as chat applications. The potential advantages of Rust include its memory safety guarantees, concurrency model, and performance, which are crucial for building scalable and efficient real-time communication systems. Rust’s memory safety without garbage collection ensures that we can achieve predictable performance without the overhead of runtime memory management, which is crucial for latency-sensitive applications. Its ownership and borrowing model allows for safe concurrency, preventing common multithreading errors like data races. Rust’s message-passing capabilities and asynchronous programming model allow us to efficiently handle large numbers of simultaneous connections, making it a great fit for network-heavy applications. Besides, its strong type system helps us catch bugs at compile time, significantly reducing runtime errors, which enhances both reliability and maintainability. We are driven by the potential to enhance user experience through these unique advantages, ensuring that our application can handle high traffic without compromising performance.

Although Rust has been gaining popularity, it is still less commonly used for building mainstream chat applications, leaving a gap in the market for a Rust-native solution. We aim to fill this gap by developing a real-time chat application written entirely in Rust for our course project. This project will not only enable us to solve real-world problems but also showcase Rust's potential in creating high-level, user-facing applications beyond its typical system programming uses.

## Objectives

The main objective of this project is to design and implement a high-performance real-time chat application using Rust, providing features such as user registration, login, chat room creation and participation, and instant message exchange. This project also intends to showcase Rust's capabilities in building a reliable and efficient communication system, with a strong focus on optimizing backend performance while maintaining a user-friendly frontend.

Considering that most Rust-based chat applications currently depend on frontend frameworks like React, with limited exploration of Rust-native frameworks for frontend development, this project aims to bridge that gap. By leveraging the Yew framework to build the chat application's user interface, the project seeks to enrich the Rust ecosystem.

The backend will use Actix Web, a powerful and scalable framework for high-performance web development. By integrating Actix Web with WebSocket, the application could efficiently handle multiple concurrent client connections, ensuring low-latency, real-time communication even under heavy user loads. This combination guarantees the application remains responsive and reliable, providing a seamless messaging experience. Additionally, the backend will leverage SurrealDB, a modern database solution, for message persistence and history retrieval.

Ultimately, the project aims to create a fully functional chat application written entirely in Rust, showcasing the language's potential for building more than just system-level software. By connecting a high-performance backend with a responsive, user-friendly frontend, this project explores how Rust can be used to create innovative web applications and contribute to its growing adoption in the industry.

## Features

#### 3. Chat room creation and joining
Users can create new chat rooms with unique room numbers, allowing others to join and engage in conversations. Each chat room operates independently, ensuring that messages sent within a specific room are visible only to its participants. This isolation enhances privacy and minimizes distractions, enabling users to focus on their discussions. By providing dedicated spaces for communication, this feature empowers users to engage in their favorite topics with ease and efficiency.

#### 4. Real-time messaging using WebSockets
The application utilizes WebSocket technology, a protocol designed to establish a persistent, full-duplex connection between clients and the server. Unlike traditional HTTP, WebSocket supports continuous, two-way communication without the overhead of repeatedly opening and closing connections. This enables the application to transmit messages instantly, ensuring a smooth and interactive user experience. By facilitating immediate message delivery, the system offers a dynamic and responsive platform for efficient and real-time communication.

#### 5. User Online/Offline status detection
The system includes User Online/Offline status detection to show whether a user is online or offline in a chat room. This functionality helps improve user engagement by enabling participants to see who is currently active in the conversation.

#### 6. Message persistence and history retrieval
To ensure a reliable communication experience, all chat room messages are stored in a database, enabling both persistence and easy access to message history. This feature prevents message loss caused by unexpected disconnections or browser refreshes. Upon re-entering a chat room, users can retrieve previously exchanged messages, seamlessly resuming their conversations without missing any context. With the help of SurrealDB, the message retrieval process is fast and reliable, enhancing the user experience by providing consistent and efficient access to message history

#### 7. Frontend user interface
A simple and responsive interface has been developed, enabling users to register, log in, create or join chat rooms, and exchange messages in real time. Built with the Yew framework, the design prioritizes ease of use and smooth navigation. Key features such as user authentication, room creation, and message display are fully integrated. The frontend works seamlessly with the backend, ensuring a unified and efficient chat experience.



