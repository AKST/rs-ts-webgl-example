main() {
  local webpack_pid=?
  ( ./node_modules/.bin/webpack -d --watch & ); webpack_pid=$!

  node server.js

  kill $webpack_pid
}


main
