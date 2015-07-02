#!/usr/bin/env escript

-import(lists, [all/2, seq/2]).

% Prime number calculation in Erlang
% Author: Kevin Seidel <keseidel@uos.de>
%
% echo -n 'p(N)->all(fun(X)->(N rem X)/=0 end,seq(2,N-1)).'
% | wc -c
%
% -> 47
%
p(N)->all(fun(X)->(N rem X)/=0 end,seq(2,N-1)).

% Print primes to stdout
main(_) ->
	io:format( "~p~n", [[ X || X <- lists:seq(2, 100), p(X) ]] ).
