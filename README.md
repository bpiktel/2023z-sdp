# Aplikacja do przeprowadzania testów lokalizacji

## Architektura

Aplikacja oparta jest o standardową architekturą 3-warstwową składającą się z frontendu, backendu i bazy danych.
Przygotowany plik docker-compose.yml uruchamia 2 kontenery: backend (który serwuje statyczne pliki frontendu) oraz bazę danych.

Frontend napisany został w języku TypeScript z użyciem frameworka React.
Backend napisany został w języku Rust z użyciem frameworka Axum.
Zastosowaną technologią bazy danych jest SurrealDB.

## Przygotowanie do uruchomienia

Przed uruchomieniem produkcyjnym należy przygotować klucze RSA służące do podpisu i walidacji tokenów JWT oraz podać dane logowania administratora.
Konto administratora tworzone jest **przy pierwszym uruchomieniu** aplikacji, więc zmiana podanych danych administratora w kolejnych uruchomieniach nie przyniesie skutku.

### Docker compose

Na podstawie pliku szablonu `docker-compose.yml.template` należy stworzyć plik `docker-compose.yml` - poprzez jego skopiowanie:
```sh
cp docker-compose.yml.template docker-compose.yml
```
a następnie edycję zgodnie z dalszymi instrukcjami.
Utworzony plik `docker-compose.yml` będzie zawierał wrażliwe dane produkcyjne i z tego powodu jest wyłączony z systemu kontroli wersji.

### Generowanie kluczy szyfrujących używanych w środowisku produkcyjnym

1. Generowanie pary kluczy:
```sh
mkdir keys # Folder `keys` jest wyłączony z systemu kontroli wersji
cd keys
ssh-keygen -t rsa -b 4096 -m PEM -f jwt-auth-rsa.key -N "" # Bez passphrase
openssl rsa -in jwt-auth-rsa.key -pubout -outform PEM -out jwt-auth-rsa.key.pub
cd ..
```

2. Podpięcie folderu z kluczami do obrazu (w `docker-compose.yml`):
```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    volumes:
      - ./keys:/app/keys
```

3. Podanie ścieżek do konkretnych plików klucza (w `docker-compose.yml`):
```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    environment:
      auth_keys__encoding: keys/jwt-auth-rsa.key
      auth_keys__decoding: keys/jwt-auth-rsa.key.pub
```

### Ustawianie danych administratora

Uwaga: dane administratora są ustawiane tylko przy pierwszym uruchomieniu! (na czystej bazie danych)

W `docker-compose.yml`:

```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    environment:
      admin__username: <wypełnić>
      admin__password: <wypełnić>
```

### (opcjonalnie) zmiana portu

Należy zamienić

```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    ports:
      - 80:3000
```

na

```
services:
  sound-localization-tester-backend-frontend:
    # [...]
    ports:
      - <nowy port>:3000
```

### Uruchomienie

Podstawowy scenariusz użycia sprowadza się do następujących poleceń:

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

Po uruchomieniu aplikacja będzie dostępna pod adresem `http://localhost:80` (jeżeli został domyślny port).
