# Задание 1

# Эмулятор оболочки ОС

Этот проект представляет собой эмулятор оболочки ОС, который работает с виртуальной файловой системой, запакованной в формате zip. Эмулятор поддерживает несколько команд и запускается из реальной командной строки.

## Требования

- Rust
- Cargo

## Установка

Скачайте или клонируйте репозиторий:

```sh
git clone <URL вашего репозитория>
```

Убедитесь, что у вас установлен Rust и Cargo.

## Виртуальная файловая система

Создайте архив `vfs.zip`, содержащий виртуальную файловую систему. Пример структуры архива:

```
├── startup.sh
├── testfile.txt
└── dir1
    └── file1.txt
```

Пример содержимого файлов:

`startup.sh`:

```
ls /
uname
```

`testfile.txt`:

```
line1
line2
line3
```

`dir1/file1.txt`:

```
This is a test file in dir1.
```

## Запуск эмулятора

Запустите эмулятор, указав путь к файлу конфигурации:

```sh
cargo run --release
```

## Команды

Эмулятор поддерживает следующие команды:

- `ls`: Список файлов и директорий.
- `cd`: Смена текущей директории.
- `clear`: Очистка экрана.
- `chmod`: Изменение прав доступа к файлам.

## Тестирование

Для запуска тестов используйте следующую команду:

```sh
cargo test
```

Тесты проверяют работу команд `ls`, `cd`, `chown`, `whoami`.