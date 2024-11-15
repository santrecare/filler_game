# PHP player template

To start your player
```
docker network create arena-network
docker-compose up -d
```

----

To develop your player, you can open the code in your favorite text editor and modify the function `eval()` in `src/eval.php`
```
docker-compose exec -it php_player_template bash
PLAYER_NAME=php_player_test php /app/main.php
```

---

To stop everything
```
docker-compose down
docker network rm arena-network
```
