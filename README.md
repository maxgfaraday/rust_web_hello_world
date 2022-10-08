# rust_web_hello_world

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/maxgfaraday/rust_web_hello_world/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/maxgfaraday/rust_web_hello_world/tree/main)
<p>
Learning Rust - one step at a time

Building a simple web service with rust

This follows the Rust book [Zero To Production](https://www.zero2prod.com/)
I suggest you purchase the book and support this really really helpful resource.
You may also follow along on-line via [Luca's blog](https://www.lpalmieri.com/) that captures much of the [content of the book](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/#:~:text=Book%20%2D%20Table%20Of%20Contents).

This project and README is me following Luca's book and taking notes about my experience and recording salient points that may make another explorer's journey, maybe a bit easier.

## Requiremenets

* [Rust](https://www.rust-lang.org/) (Cargo, rustup) *of course*
* [PostgreSQL](https://www.postgresql.org/)
* [Docker](https://docs.docker.com/) and [Docker Hub](https://hub.docker.com/) account (*I recommend [start here](https://www.docker.com/get-started/) if not familiar*)
* [GitHub](https://github.com/) account
* [Digital Ocean](https://www.digitalocean.com/) account
* [Circle CI](http://circleci.com) account
* [Curl](https://curl.se/)

*Note - I chose to use CircleCI vs GitHub Actions, all things being equal I would have chosen GitHub Actions. However my favorite language is [Clojure](https://clojure.org/) so I am of course compelled to support my language mates! Go Clojure!!*

Since we are building a very simple subscription **web** service, you speak to it via http, so of course we use [Curl](https://curl.se/) to spit data at it.  Here are two curl commands to spit input data at our service's simple API. (Good for quick local sanity checking. No worries this gets formalized in our explicit tests - we are not barbarians!)

``` bash
%> curl -v http://127.0.0.1:8000/health_check
%> curl -i -X POST -d 'email=cynthia.rose230@hotmail.com&name=Cynthia' http://127.0.0.1:8000/subscriptions
```

---

## Database (Postgres)

We persist the data in the system via [PostgreSQL](https://www.postgresql.org/).  What is needed for the purposes of this project is actually only the `psql` tool. To get `psql` you can install postgres locally with `brew install postgres` (yes, I am on a mac).  You do not need the full Postgres engine instance running directly because the running Postgres we use for this project is [run via a Docker container](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/#:~:text=To%20run%20Postgres%20we%20will%20use%20Docker).

*Note - If you `brew install postgres` it may also fire up the postgres instance on your bare metal directly.  This may interfere (port conflict, etc) with the containerized postgres that we usw in this project. So you must to turn off the postgres engine installed via the brew command by issuing the following:

``` bash
brew services stop postgresql
```
([solution hint](https://stackoverflow.com/questions/34173451/stop-postgresql-service-on-mac-via-terminal#:~:text=brew%20services%20stop%20postgresql))

We create our actual database instance and apply the schema via *migrations* (because we are not barbarians) and we do so through the tool `sqlx` - we get `sqlx` via `cargo`. (as the project progresses we eschew using sqlx as a command line tool and provide the same migration capabilities in code.)

Misc:
``` bash
%> sqlx migrate add create_subscriptions_table
```
(see: [here](https://www.lpalmieri.com/posts/2020-08-31-zero-to-production-3-5-html-forms-databases-integration-tests/#:~:text=sqlx%20migrate%20add%20create_subscriptions_table))


## Running in Docker

This service in this project is wrapped in a Docker container. I have posted one [here](https://hub.docker.com/r/maxgfaraday/z2p/tags)

To build the container I used:

``` bash
docker build --tag maxgfaraday/z2p:99ddb9e --file Dockerfile .
```

(But of course for you, you will use your own tag coordinates)

To run the container use:

``` bash
%> docker run -it --rm -p 8000:8000 maxgfaraday/z2p
```

And of course to do rudimentary poking to check that everything is sane, `curl` some data at it (see above).

## Clouds n Repos n Containers, Oh My!

*Digital Ocean* <br>

When setting up Digital Ocean as the cloud provider there was a bit of an issue with getting [doctl](https://github.com/digitalocean/doctl) to initialize and use my accesstoken.  The fix was to go into the location were doctl was looking for its config, there may already be a config.yaml file there but nuke it and reinitialize. The reinitialization will create a new one that will be better written to record the access token properly.

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

## Trouble connecting DO to Github

Problem: It is exactly described [here](https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/#:~:text=can%20try%20again%3A-,doctl%20apps%20create%20%2D%2Dspec%20spec.yaml,OK%2C%20follow%20their%20instructions%20to%20link%20your%20GitHub%20account.,-Third%20time%27s%20a), but as of this date I have not been able to surmount it despite following the information pointed to by the provided link.
