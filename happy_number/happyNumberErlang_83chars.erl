#!/usr/bin/env escript

% Happy number calculation in Erlang
% Author: Lars Kiesow <lkiesow@uos.de>
%
% echo -n 'd(0)->0;d(X)->(X rem 10)*(X rem 10)+d(X div 10).
% h(4)->1>2;h(1)->1>0;h(X)->h(d(X)).'|wc -c
%
% -> 83
%
d(0)->0;d(X)->(X rem 10)*(X rem 10)+d(X div 10).
h(4)->1>2;h(1)->1>0;h(X)->h(d(X)).

% Print happy numbers to stdout
main(_) ->
	io:format( "~p~n", [[ X || X <- lists:seq(1, 100), h(X) ]] ).
