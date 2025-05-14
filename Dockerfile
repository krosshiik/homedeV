# Используем официальный образ Rust
FROM rust:1.82

# Устанавливаем рабочую директорию
WORKDIR /usr/src/app

# Копируем Cargo.toml и Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Создаем временный main.rs, чтобы cargo fetch не упал
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Кэшируем зависимости
RUN cargo fetch

# Удаляем временный src/
RUN rm -r src

# Копируем весь остальной проект
COPY . .

# Сборка
RUN cargo build --release

# Открываем порт
EXPOSE 3005

# Команда запуска
CMD ["./target/release/rust-ci-app"]
