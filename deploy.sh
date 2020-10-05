cargo run --release
npm run prod
rsync -Pr --delete build/ thibaud.dauce.fr:/var/www/thibaud.dauce.fr/
