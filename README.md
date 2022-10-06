# rust_web_hello_world

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/maxgfaraday/rust_web_hello_world/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/maxgfaraday/rust_web_hello_world/tree/main)
<p>
Learning Rust - one step at a time

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

## Clouds n Repos n Containers, Oh My!

*Digital Ocean* <br>

When setting up Digital Ocean as the cloud provider there was a bit of an issue with getting [doctl](https://github.com/digitalocean/doctl) to initialize and use my accesstoken.  The fix was to go into the location were doctl was looking for its config, there may already be a config.yaml file there but nuke it and reinitialize. The reinitialization will create a new one that will be better written to hold the access token properly.

``` bash
%> pushd ~/Library/Application\ Support/doctl
%> mv ~/Library/Application\ Support/doctl/config.yaml{,.old}
%> touch ~/Library/Application\ Support/doctl/config.yaml
%> doctl auth init

Using token [dop_v1_wwwwxxxxxxxxxxxxxxxxxxxyyyyyyyyyyyyyyyyyyyyyzzzzzzzzzzzzzzzzzzz]

Validating token... OK
```

That means you are all good. Just for a sanity check docker

``` bash
%> doctl auth list
<should give list of projects>

%> doctl account get
<should see entry>
```
