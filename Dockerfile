FROM rust:1.60

RUN apt update && apt install libzmq3-dev -y

WORKDIR /home
COPY . .

RUN cargo build --release
RUN cargo build --release --example async_publisher

ENV PATH="/home/target/release/examples/:${PATH}" 

EXPOSE 7800
EXPOSE 8000
CMD ["async_publisher"]