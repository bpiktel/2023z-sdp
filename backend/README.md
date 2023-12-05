# Aplikacja backend

Dostarcza proste API oraz tymczasową persystencje audio i danych eksperymentów.

## Uruchomienie (do testowania frontendu)

```sh
# Uruchom
docker compose up -d
# Przebuduj i uruchom (przy aktualizacji kodu źródłowego)
docker compose up --build -d
# Zatrzymaj
docker compose down
# Zatrzymaj i usuń dane
docker compose down -v
```

API będzie dostępne pod `http://localhost:3000`.
