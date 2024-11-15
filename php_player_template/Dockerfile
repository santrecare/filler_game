FROM composer:latest

RUN apk add --no-cache \
    php \
    php-cli \
    php-zip \
    php-mbstring \
    git \
    && apk add --no-cache --virtual .build-deps \
    libzip-dev \
    && apk del .build-deps
COPY composer.json /app/
RUN composer install --no-dev
COPY src/ /app/

CMD ["php", "/app/main.php"]
