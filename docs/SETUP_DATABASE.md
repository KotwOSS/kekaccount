# Setup Database

## Creating

1. Create database:
```sql
CREATE DATABASE account;
```
2. Create user:
```sql
CREATE USER account 
WITH PASSWORD '<password>';
```
3. Grant privileges:
```sql
GRANT ALL PRIVILEGES ON DATABASE account TO account;
```

<br>

## Removing

1. Drop user:
```sql
DROP USER account;
```

2. Drop database:
```sql
DROP DATABASE account;
```