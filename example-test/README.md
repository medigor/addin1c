# example-test

Тесты компоненты для использования с `valgrind`, пример запуска:
```bash
cargo build && valgrind --leak-check=full --show-leak-kinds=all target/debug/example-test
```