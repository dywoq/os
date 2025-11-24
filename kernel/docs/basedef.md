# basedef.md 

Base definitions of types used across the kernel.

## Integral types

```c
typedef signed char      SCHAR;
typedef signed short     SSHORT;
typedef signed int       SINT;
typedef signed long long SLONGLONG;
typedef unsigned char      UCHAR;
typedef unsigned short     USHORT;
typedef unsigned int       UINT;
typedef unsigned long long ULONGLONG;
typedef ULONGLONG          UINTPTR;
```

# Types related to strings

```c
typedef const char    *ASCII_STR;
```
