all:
		@echo Run \'make install\' to install power-calc.

install:
		@cargo install --path ./power-calc/

uninstall:
		@cargo uninstall power-calc
