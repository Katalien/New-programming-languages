FROM erlang:24

WORKDIR /usr/src/app

COPY . .

CMD ["erl", "-noshell", "-s", "client", "start", "server-host"]
