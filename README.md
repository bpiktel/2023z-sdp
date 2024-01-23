# Aplikacja do przeprowadzania testów lokalizacji

## Architektura

Aplikacja jest standardową architekturą 3-wartswową: frontend, backend, baza danych.
Aplikacja podzielona jest na 2 kontenery: backend, który serwuje statyczne pliki frontendu oraz bazę danych.

Frontend wykorzystuje framework React w języku TypeScript.
Backend wykorzystuje framework Axum w języku Rust.
Baza danych to SurrealDB.

## Przygotowanie do uruchomienia

Przed uruchomieniem produkcyjnym należy przygotować klucze RSA służące do podpisu i walidacji tokenów JWT oraz podać dane logowania administratora.
Podane dane administratora są wykorzystywane tylko **przy pierwszym uruchomieniu** aplikacji.

### Docker compose

Plik `docker-compsoe.yml.template` należy skopiować i usunąć człon `.template`.
Zostanie on wypełniony danymi produkcyjnymi i jest wyłączony z kontroli wersji.

### Generacja kluczy produkcyjnych JWT

1. Wygenerować klucz:
```sh
mkdir keys # Folder `keys` jest wyłączony z kontroli wersji
cd keys
ssh-keygen -t rsa -b 4096 -m PEM -f jwt-auth-rsa.key # Bez passphrase
openssl rsa -in jwt-auth-rsa.key -pubout -outform PEM -out jwt-auth-rsa.key.pub
cd ..
```

2. Podpiąć ścieżkę do klucza do obrazu:
```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    volumes:
      - ./keys:/keys
```

3. Podać ścieżki do konkretnych plików
```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    environment:
      auth_keys__encoding: keys/jwt-auth-rsa.key
      auth_keys__decoding: keys/jwt-auth-rsa.key.pub
```

### Ustawianie danych administratora

Dane administratora są ustawiane tylko przy pierwszym uruchomieniu! (na czystej bazie danych)

```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    environment:
      admin__username: <wypełnić>
      admin__password: <wypełnić>
```

### Uruchomienie

Najprostsza obsługa sprowadza się do następujących poleceń:

```sh
# Uruchom
docker compose up -d
# Przebuduj (ważne przy aktualizacji) i uruchom
docker compose up --build -d
# Zatrzymaj
docker compose down
# Zatrzymaj i usuń dane (konto administratora, audio, eksperymenty, wyniki)
docker compose down -v
```

Po uruchomieniu aplikacja będzie dostępna pod `http://localhost:3000`.
