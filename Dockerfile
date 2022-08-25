 
# Se emplea Ubuntu 18.04 como base ya que es menor que la version 20.04
FROM ubuntu:18.04

# Variables de entorno para rustup
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.58.0

# 1. Instala dependencias y paquetes necesarios para rustup
# 2. Descarga rustup para GNU/Linux x86_64 y da pertmisos y ejecuta.
# 3. Da permisos de lectura a los usuarios en los directorios de rustup y cargo
# 4. Crea usuario beziervice

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \
    make \
    ; \
    wget "https://static.rust-lang.org/rustup/archive/1.22.1/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host x86_64-unknown-linux-gnu; \
    useradd  berziervice; 


# Directorio de trabajo donde se montara el repositorio
WORKDIR /app/test

# Al inicio del contenedor
# 1. Da ownership al usuario beziervice en el WORKDIR
# 2. Ejecuta make test
CMD chown -R berziervice /app/test && su berziervice -c " PATH=/usr/local/cargo/bin:$PATH make test"