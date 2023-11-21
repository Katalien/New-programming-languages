-module(server).
-export([start/0, loop/0, handle_client/1]).

start() ->
    spawn(fun() -> loop() end).

loop() ->
    {ok, ListenSocket} = gen_tcp:listen(12345, [binary, {packet, 4}, {reuseaddr, true}]),
    accept_clients(ListenSocket).

accept_clients(ListenSocket) ->
    {ok, ClientSocket} = gen_tcp:accept(ListenSocket),
    spawn(fun() -> handle_client(ClientSocket) end),
    accept_clients(ListenSocket).

handle_client(ClientSocket) ->
    case gen_tcp:recv(ClientSocket, 0) of
        {ok, Data} ->
            io:format("Received: ~s~n", [Data]),
            gen_tcp:send(ClientSocket, <<"Server received: ", Data/binary>>),
            handle_client(ClientSocket);
        {error, closed} ->
            io:format("Client closed the connection.~n", []),
            ok
    end.
