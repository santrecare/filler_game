# Utiliser Debian Buster (plus ancien) pour la compilation
FROM rust:1.75-buster as builder

WORKDIR /usr/src/app
COPY . .

# Compiler avec les flags pour une compatibilité maximale
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

# Utiliser Debian Buster pour l'image finale
FROM debian:buster-slim

# Installer les dépendances minimales nécessaires
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copier le binaire
COPY --from=builder /usr/src/app/target/release/arena /usr/local/bin/

CMD ["arena"]
