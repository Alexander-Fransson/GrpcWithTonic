# General information abot proc-macros

Proc macros are run at compile time so...
* dont run system commands, 
* dont access files, 
* dont use usnafe code
* dont allow users to inject code
* only use trusted dependencies
* dont read env variables
* restrict the input types

Also error handling is impossible in macros, you can only panic or use compiler_error.
Furthermore, proc macros are so called unhygenic which means that they will be compiled as raw code in the context they are used. This means that you need to make sure they can be used in as many contexts as possible, for example use ::syn::option::Option<T> instead of Option<T>, unique function names and absolute file paths(except dont access files)