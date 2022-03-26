# Token permissions

## Bit mask

| bit | name             | description                   |
| --- | ---------------- | ----------------------------- |
| 0   | token_list       | List tokens                   |
| 1   | app_create       | Create apps                   |
| 2   | app_delete       | Delete apps                   |
| 3   | app_update       | Update apps                   |
| 4   | app_list         | List apps                     |
| 5   | user_info        | Show private self information |
| 6   | user_update      | Update self user              |
| 7   | app_token_create | Create app tokens             |
| 8   | app_token_delete | Delete app tokens             |
| 9   | app_token_list   | List app tokens               |
| 10  | token_terminate  | Terminate self token          |

# App token permissions

## Bit mask

| bit | name         | description                            |
| --- | ------------ | -------------------------------------- |
| 0   | tcp_api      | Access to tcp api                      |
| 1   | access_token | Convert access code to an access token |
