alias w := watch

watch: 
	cargo watch -x 'run --bin routing'

style:
	npm run style
