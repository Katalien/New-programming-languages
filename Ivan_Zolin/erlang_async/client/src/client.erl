-module(client).
-export([start/1]).

start(ServerHost) ->
    {ok, Socket} = gen_tcp:connect(ServerHost, 12345, [binary, {packet, 4}]),
    io:format("Connected to server.~n"),
    loop(Socket).

loop(Socket) ->
    {ok, Line} = io:get_line("Enter message (or 'exit' to quit): "),
    case Line of
        "exit\n" ->
            gen_tcp:close(Socket),
            io:format("Connection closed.~n");
        _ ->
            gen_tcp:send(Socket, Line),
            {ok, Response} = gen_tcp:recv(Socket, 0),
            io:format("Server response: ~s~n", [Response]),
            loop(Socket)
    end.
