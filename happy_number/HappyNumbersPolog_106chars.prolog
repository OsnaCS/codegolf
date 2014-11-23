%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%                      106 CHARS                           %%
%% This is a SWI-Prolog program that caluculates if a given %%
%% Number is happy or not.                                  %%
%% Start Program with h(Z) where Z is number in question.   %%
%% author: Christian Heiden (cheiden@uos.de)                %%
%%                                                          %%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

s(0,E,E).
s(X,T,E):-M is X mod 10,Y is M^2,S is Y+T,Z is X div 10,s(Z,S,E).
h(1).
h(Z):-Z\=4,s(Z,0,E),!,h(E).
