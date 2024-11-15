# Python player template

To start your player
```
docker network create arena-network
docker-compose up -d
```

----

To develop your player, you can open the code in your favorite text editor and modify the function `eval()` in `src/eval/eval.py`
```
docker-compose exec -it python_player_template bash
python -um src.main --player_name python_player_test
```

---

To stop everything
```
docker-compose down
docker network rm arena-network
```

