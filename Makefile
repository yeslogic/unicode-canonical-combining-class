UCD:=16.0.0

tables:
	yeslogic-ucd-generate canonical-combining-class --rust-enum ../ucd-generate/ucd-$(UCD) > src/tables.rs
	cargo fmt


.PHONY: tables

