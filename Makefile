.PHONY: release syntax test examples clean

EXEC = './target/release/chalcedony'

release: Cargo.toml ./src ./src/main.rs
	if [ -z $(shell which cargo) ]; then \
		echo 'Error: you need to have cargo installed'; \
		exit 1; \
	fi
	if [ ! -w '/usr/local/bin' ]; then \
		echo 'Error: no permission to access "usr/local/bin"'; \
		exit 1; \
	fi
	cargo build --release
	cargo clippy || true
	cp ./target/release/chalcedony /usr/local/bin/chal

# adds syntax highlighting for *.ch files
syntax: ./utils/syntax/chal.vim
	# for vim:
	mkdir -p ~/.vim/syntax 
	cp ./utils/syntax/chal.vim ~/.vim/syntax/ch.vim
	mkdir -p ~/.vim/ftdetect
	touch ~/.vim/ftdetect/ch.vim
	echo "au BufRead,BufNewFile *.ch set filetype=chalcedony" > ~/.vim/ftdetect/ch.vim
	echo "au BufRead *.ch setlocal shiftwidth=4 softtabstop=4" >> ~/.vim/ftdetect/ch.vim

	# for nvim:
	mkdir -p ~/.config/nvim/syntax 
	cp ./utils/syntax/chal.vim ~/.config/nvim/syntax/chalcedony.vim

test:
	cargo test --features testing

examples:
	if [ ! -f  ${executable} ]; then \
		echo "Error: the ${executable} does not exist"; \
		exit 1; \
	fi
	for file in $(shell find ./examples -type f -printf "%f\n"); do \
		echo "Running example script: $${file}"; \
		${EXEC} ./examples/$${file} 1> /dev/null || true; \
	done

clean:
	cargo clean
