set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

build:
  just puniyu/build
build-core:
  just puniyu_node_core/build
run:
  just puniyu/run