# ECE1724 Course Project - Final Report

| Student Name|  Student ID |  
|-------------|-------------|
| Zhaoyu (Selena) Yan  | 1006521621 |
| Yiran (Sheila) Chen | 1004392739 |
| Yixu (Henry) Ye  | 1010506579 |

## Motivation

Real-time communication is central to our digital lives, shaping everything from work meetings to casual catch-ups with friends. While many established chat applications exist, they often face performance and scalability challenges under high user loads. Several of these platforms may struggle to provide low-latency communication and a robust infrastructure for thousands of concurrent users.

Motivated by these common issues, our team has decided to address them in our project by leveraging the unique strengths of Rust. While domains like web and network application development are typically dominated by languages like JavaScript and Python—each with mature ecosystems and extensive tooling—Rust offers standout features that make it an ideal candidate for building real-time systems such as chat applications. The potential advantages of Rust include its memory safety guarantees, concurrency model, and performance, which are crucial for building scalable and efficient real-time communication systems. Rust’s memory safety without garbage collection ensures that we can achieve predictable performance without the overhead of runtime memory management, which is crucial for latency-sensitive applications. Its ownership and borrowing model allows for safe concurrency, preventing common multithreading errors like data races. Rust’s message-passing capabilities and asynchronous programming model allow us to efficiently handle large numbers of simultaneous connections, making it a great fit for network-heavy applications. Besides, its strong type system helps us catch bugs at compile time, significantly reducing runtime errors, which enhances both reliability and maintainability. We are driven by the potential to enhance user experience through these unique advantages, ensuring that our application can handle high traffic without compromising performance.

Although Rust has been gaining popularity, it is still less commonly used for building mainstream chat applications, leaving a gap in the market for a Rust-native solution. We aim to fill this gap by developing a real-time chat application written entirely in Rust for our course project. This project will not only enable us to solve real-world problems but also showcase Rust's potential in creating high-level, user-facing applications beyond its typical system programming uses.

## Objectives

The objective of this project is to design and implement a high-performance real-time chat application in Rust, enabling users to register, log in, create or join chat rooms, and exchange messages instantly. This application will serve as a demonstration of Rust's capabilities in building a reliable and efficient communication system, with a strong emphasis on enhancing backend performance while ensuring frontend usability.

Considering that most chat applications built with Rust currently rely on frontend frameworks like React, and there has been limited exploration into using Rust-related frameworks for frontend development. This project aims to leverage the Yew framework to create the user interface of the chat application, thereby, filling a gap in the current Rust ecosystem. The backend will be developed using Actix-Web. Ultimately, the goal is to create a fully functional chat application using nearly 100% Rust.


