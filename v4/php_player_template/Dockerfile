FROM composer:latest

# Installer PHP avec les extensions nécessaires
RUN apk add --no-cache \
    php \
    php-cli \
    php-zip \
    php-mbstring \
    git \
    && apk add --no-cache --virtual .build-deps \
    libzip-dev \
    && apk del .build-deps

# Copier votre fichier composer.json et installer les dépendances
COPY composer.json /app/
RUN composer install --no-dev

# Copier tous les fichiers PHP dans le conteneur
COPY src/ /app/

# Lancer le script principal de l'application
CMD ["php", "/app/main.php"]