# ssic

### Server Side Includes Compiler

The SHTML compiler.

Easy and minimalistic development of static websites.

This is a find and replace program that looks for SSI include directives in SHTML files, and generates HTML files for static serving.

Because it uses regular SHTML, it has the added benefit that you could upload your source files to a webserver with SSI enabled,
and get dynamic serving without needing to do any adjustments.

```
<!--#include virtual="menu.shtml" -->
```
