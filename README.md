# test sqlx native ssl

Measures a bug(?) in sqlx with native-ssl.

```shell
make init-ca
make init-db
# make up (init-db also calls make up)
./measure.sh  # optional duration in seconds as $1
# make down
massif-visualizer measurements/*
```

## system dependencies

- Valgrind with Massif
- massif-visualizer
- sqlx
- docker and docker compose
