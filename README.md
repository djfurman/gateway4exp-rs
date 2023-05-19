# gateway4exp-rs

A central broker service for exploration and testing written in Rust

## Purpose

In each tutorial about microservices or modern design, a central broker or gateway appears to be omnipresent.

In other projects I've built these with

- [NGINX](https://www.nginx.com/) as a reverse proxy
- [Amazon API Gateway](https://aws.amazon.com/api-gateway/) as a full gateway with and without serverless integrations
- the GoLang standard Library as [go-micro-broker-service](https://github.com/djfurman/go-micro-broker-service)

However, I've never built one in Rust; nor have I built one from the ground up. The best way to learn is to do!

### Disclaimer

This is a learning project, and should not be used as a production gateway. While I plan on testing this in interesting ways, my goal is **not** to harden a production grade gateway. Please do not use this as a gateway in your production workloads.

## Acknowledgements

In this project, I'll be following the evolution pattern @LukeMathWalker introduced in his [zero2prod](https://www.zero2prod.com) book. I really enjoy the writing style and encourage you to check out the title if you're interested in Rust and you learn better by experience.

## Contributing

Reiterating that this is a learning project and challenge for me, any contributions are welcome. After all, working together is what makes Open Source software fun. If you'd like to contribute, please start by opening an issue and let's discuss it in a thread. My goal here is to learn by doing. If you'd like to be part of that journey, I look forward to collaboarting! If not, I may be writing this for the ether.

I welcome your PRs, and I value your time. Starting with an issue to discuss is my way to make sure your idea isn't something I need to experiment with myself, or may go in a learning direction I'm not looking to take a the moment. There may be excellent crates that already implement the bulk of what I want to do (e.g., [hyper-reverse-proxy](https://crates.io/crates/hyper-reverse-proxy) which is a strong candidate).
