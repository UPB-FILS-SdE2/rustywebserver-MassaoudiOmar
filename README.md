[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/TXciPqtn)
# Rustwebserver

Detail the homework implementation.


Looking through this implementation, it is clearly noticable that a very naive, straight forward implementation was opted for. It does not use async, does not use tokio, does not use execv and avoids any unncessarily complex libraries and such.  

The goal is very simple, parse the HTTP request, perform the necessary operations using very linear logic (if-elseif-else). 


The response is formulated in a very straight-forward string literal. Simple logic determines whther the path is allowed, then determines whether or not it is a script. 

if it is a file, the contents are taken from the file then printed as part of the response. 

if it is a script, it is ran, then the output is printed as part of the response. 

The main idea was to get as many points as possible, using the dumbest approach possible, without hard coding anything. I believe this is achieved in a legitimate way. 
