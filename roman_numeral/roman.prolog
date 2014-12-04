%% 455 characters
%% author: Christian Heiden

%% print symbols.
p(1):-write('I').
p(2):-write('V').
p(3):-write('X').
p(4):-write('L').
p(5):-write('C').
p(6):-write('D').
p(7):-write('M').

%% rules for printing.
o(0,_).
o(1,P):-p(P).
o(2,P):-p(P),p(P).
o(3,P):-p(P),p(P),p(P).
o(4,P):-p(P),R is P+1,p(R).
o(4,P):-R is P+1,p(R).
o(5,P):-R is P+1,p(R).
o(6,P):-R is P+1,p(R),p(P).
o(7,P):-R is P+1,p(R),p(P),p(P).
o(8,P):-R is P+1,p(R),p(P),p(P),p(P).
o(9,P):-p(P),R is P+2,p(R).

%% User predicate.
r(Z):-r(Z, 1).

%% right recursive function.
r(0,_).
r(Z,P):-	A is Z mod 10,
			Y is Z div 10,
			R is P+2,r(Y,R),
			o(A,P).

