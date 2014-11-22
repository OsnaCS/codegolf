%% A small swi-prolog program to check for primenumbers.
%%
%% author: Christian Heiden
%%
%% HowTo start:
%% 1) Download swipl (swi-prolog) via apt-get or similar stuff
%% 2) cd to isPrime.pl
%% 3) Start it with the command swipl in your console. 
%% 4) Type ['isPrime']. to compile
%% 5) now enter the commands listed beneath.

%% user friendly version (49)
%% start with q(X) where X is your number in question

%% this is simply the call
q(Z):-p(Z,2).

%% this is the procedure
p(Z,Z).
p(Z,C):-not(0 is Z mod C),
		D is C+1,
		p(Z,D).
