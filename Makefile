BUILDDIR := build/linux/
TOOL := build/linux/refunct-tas
LIB := build/linux/librtil.so

.PHONY: all
all: $(TOOL) $(LIB) scripts

.PHONY: zip
zip: all
	cd build/ && cp -r linux practice-linux && zip -r practice-linux.zip practice-linux

.PHONY: clippy
clippy:
	cd rtil && cargo clippy
	cd tool && cargo clippy

.PHONY: check
check:
	cd rtil && cargo check
	cd tool && cargo check

.PHONY: $(TOOL) # always execute cargo
$(TOOL): $(BUILDDIR)
	cd tool && cargo build
	cp tool/target/debug/refunct-tas $(TOOL)

.PHONY: $(LIB) # always execute cargo
$(LIB): $(BUILDDIR)
	cd rtil && cargo build --release
	cp rtil/target/release/librtil.so $(LIB)

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

.PHONY: scripts
scripts: $(BUILDDIR)
	bash -c 'cp tool/*.re $(BUILDDIR)'

.PHONY: clean
clean:
	$(RM) -r build/
	cd tool && cargo clean
	cd rtil && cargo clean
