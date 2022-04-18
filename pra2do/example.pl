:- use_module(pra2do).

:- use_module(library(lists)).
:- use_module(library(format)).

main :-
    canvas_init('127.0.0.1':6767, Socket),
    loop(Socket, 10, 10).

loop(Socket, X0, Y0) :-
    canvas_input(Input, Socket),
    portray_clause(Input),
    (member(left, Input) -> X1 is X0 - 10; X1 = X0),
    (member(right, Input) -> X is X1 + 10; X = X1),
    (member(up, Input) -> Y1 is Y0 - 10; Y1 = Y0),
    (member(down, Input) -> Y is Y1 + 10; Y = Y1),
    Code = [fill_style("green"), fill_rect(X, Y, 100, 100), fill_style("blue"), fill_rect(200, 200, 300, 100)],
    canvas_do(Code, Socket),
    loop(Socket, X, Y).

