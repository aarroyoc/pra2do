:- module(pra2do, [canvas_init/2, canvas_do/2, canvas_input/2]).

:- use_module(library(lists)).
:- use_module(library(format)).
:- use_module(library(sockets)).
:- use_module(library(charsio)).

canvas_init(Addr:Port, Socket) :-
    socket_client_open(Addr:Port, Socket, []).

canvas_do([], Socket) :-
    portray_clause(Socket, end).
canvas_do([X|Xs], Socket) :-
    portray_clause(Socket, X),
    canvas_do(Xs, Socket).

canvas_input(Input, Socket) :-
    portray_clause(Socket, ask_input),
    read_term(Socket, Input, []).
    
