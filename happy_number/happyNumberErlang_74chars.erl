#!/usr/bin/env escript

% Happy number calculation in Erlang
% Author: Lars Kiesow <lkiesow@uos.de>
% Author: Kevin Seidel <keseidel@uos.de>
%
% echo -n 'd(0)->0;d(X)->Y=X rem 10,Y*Y+d(X div 10).
% h(X)when 5>X->X<2;h(X)->h(d(X)).' | wc -c
%
% -> 74
%
d(0)->0;d(X)->Y=X rem 10,Y*Y+d(X div 10).
h(X)when 5>X->X<2;h(X)->h(d(X)).

% Print happy numbers to stdout
main(_) ->
	io:format( "~p~n", [[ X || X <- lists:seq(1, 100), h(X) ]] ).
