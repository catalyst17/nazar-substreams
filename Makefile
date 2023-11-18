ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 18597208
STOP_BLOCK ?= +10

SINK_DB_NAME ?= 
SINK_DB_URL ?= 
SINK_DB_PASS ?= 

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: setup-sink
setup-sink:
	substreams-sink-sql setup "psql://$(SINK_DB_NAME):$(SINK_DB_PASS)@$(SINK_DB_URL)?sslmode=disable" ./sink/substreams.dev.yaml

.PHONY: sink
sink: build
	substreams-sink-sql run "psql://$(SINK_DB_NAME):$(SINK_DB_PASS)@$(SINK_DB_URL)?sslmode=disable" ./sink/substreams.dev.yaml

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_filter_transactions -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
