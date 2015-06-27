#!/usr/bin/env escript

% Prime number calculation in Erlang
% Author: Kevin Seidel <keseidel@uos.de>
%
% echo -n 'p(N)->lists:all(fun(X)->(N rem X)/=0 end,lists:seq(2,N-1)).'
% | wc -c
%
% -> 59
%
p(N)->lists:all(fun(X)->(N rem X)/=0 end,lists:seq(2,N-1)).

% Print primes to stdout
main(_) ->
	io:format( "~p~n", [[ X || X <- lists:seq(2, 100), p(X) ]] ).
