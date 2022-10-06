# rust_web_hello_world
Learning Rust - one step at a time
[![CircleCI](https://dl.circleci.com/status-badge/img/gh/maxgfaraday/rust_web_hello_world/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/maxgfaraday/rust_web_hello_world/tree/main)

Building a simple web service with rust

This follows the Rust [Zero To Production](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/) book

Notes:

Regarding the database.  This command is out-of-band from anything recorded.

``` bash
sqlx migrate add create_subscriptions_table
```
(see: [here](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/#:~:text=sqlx%20migrate%20add%20create_subscriptions_table))

Also it seems you have to get postgres (via brew) just to get psql.
Then there will be a conflict between the one you installed with brew and the one that is running in the container. So you have to turn off the one you got via brew with the command:

``` bash
brew services stop postgresql
```
(see: [here](https://stackoverflow.com/questions/34173451/stop-postgresql-service-on-mac-via-terminal#:~:text=brew%20services%20stop%20postgresql))
