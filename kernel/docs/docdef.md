# docdef.h

Definitions of macros, types etc. whose purpose for code is documentation-only 

## IN 
```c 
#define IN
```
The function will only read the parameter.

## OUT
```c 
#define OUT
```
The function will change the parameter.

## OPT
```
#define OPT
```
It's optional to provide the parameter to the function,
and you can mark it as a nullptr.
