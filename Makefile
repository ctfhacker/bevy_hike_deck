run:
	cargo run 

flow:
	cargo run --features flow > flow.dot
	dot -Tsvg flow.dot -oflow.svg
	xdg-open flow.svg
