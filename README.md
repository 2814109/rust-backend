# rust-postgres

## command on docker container of rust-app

- curl -v -H "Content-Type: application/json" -X POST -d '{"first_name": "foo1", "last_name": "bar1", "email": "foo1@bar.com"}' 0.0.0.0:9000/users

- curl -v 0.0.0.0:9000/users

- export TOKEN=

## ログアウト処理

AUTH0_RETURN_TO_URL と Allowed Logout URLs 　を一緒にする必要がある

## form method

get 以外の処理 patch delete post は method = "post" と宣言する
