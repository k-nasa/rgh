# x86_64-unknown-linux-gnu
# x86_64-apple-darwin
# x86_64-pc-windows-gnu

TARGET:= x86_64-pc-windows-gnu
BIN_NAME:=rgh.exe
CRATE_NAME:=rgh
MISC:= README.md LICENSE
DIRNAME:=${CRATE_NAME}_${TARGET}

release_all:
	make release TARGET=x86_64-pc-windows-gnu    BIN_NAME=rgh.exe
	make release TARGET=x86_64-apple-darwin      BIN_NAME=rgh
	make release TARGET=x86_64-unknown-linux-gnu BIN_NAME=rgh

.PHONY: release
release:
	cross build --target ${TARGET} --release
	mkdir -p ${DIRNAME}
	\
	cp ./target/${TARGET}/release/${BIN_NAME} ${DIRNAME}
	cp ${MISC} ${DIRNAME}
	\
	mkdir -p dist
	tar czf dist/${DIRNAME}.tar.gz ${DIRNAME}
	rm -rf ${DIRNAME}
