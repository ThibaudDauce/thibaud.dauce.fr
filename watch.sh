while true; do
  cargo run --release &
  inotifywait -re close_write,moved_to,create src content --exclude content/css/syntax.css
done
