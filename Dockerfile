FROM rust:latest

WORKDIR /redis_demo

RUN apt update
RUN apt install -y fish vim redis-tools python3-redis

# Keep container running
CMD ["tail", "-f", "/dev/null"]
