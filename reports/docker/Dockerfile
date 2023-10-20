FROM rust:1.71.0

RUN apt-get update && \
    apt-get install mc -y && \
    useradd -rm -d /home/student -s /bin/bash -g root -u 1000 student

USER student
WORKDIR /home/student

CMD exec /bin/bash -c "trap : TERM INT; sleep infinity & wait"
