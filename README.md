# addin1c
Helper for creating 1C:Enterprise 8 add-ins with Native API technology

References:
* [1C:Enterprise Guide (ru)](https://its.1c.ru/db/metod8dev#content:3221:hdoc)
* [1C:Enterprise Guide (en)](https://kb.1ci.com/1C_Enterprise_Platform/Guides/Developer_Guides/Extra/Add-in_Development_Technology/Creating_add-ins_with_Native_API_technology/)

See [example](example):
* [addin1.rs](example/src/addin1.rs) - raw interface
* [addin2.rs](example/src/addin2.rs) - simple interface

## Debugging in Visual Studio Code
Create a `.vscode/launch.json` file:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug 1С",
            "program": "path/to/1cv8c",
            "args": [
                "/IBName",
                "Test1"
            ],
            "cwd": "${workspaceFolder}",
            "preLaunchTask": "rust: cargo build",
            "env": {"DISPLAY": ":1"}, // only for Linux
        }
    ]
}
```
