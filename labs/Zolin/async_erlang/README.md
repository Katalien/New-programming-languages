# Async chat lab on Erlang (Ivan Zolin)

## Deployment

```bash
docker build -t erlang-chat-server -f Dockerfile.server .
docker build -t erlang-chat-client -f Dockerfile.client .
docker run -it erlang-chat-server
docker run -it erlang-chat-client
docker run -it --rm --name chat-client erlang-chat_image # in new terminal
```
