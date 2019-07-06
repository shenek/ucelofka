SRC_DIR := invoices
OUT_DIR := output

SRC_FILES := $(wildcard $(SRC_DIR)/*.yml)
HTML_FILES := $(patsubst $(SRC_DIR)/%.yml,$(OUT_DIR)/%.html,$(SRC_FILES))

.phony: clean

$(OUT_DIR)/%.html: $(SRC_DIR)/%.yml
	ucelofka invoice render -T default.html -I $(notdir $(basename $<))


html: $(HTML_FILES)


all: html


clean:
	rm -rf $(OUT_DIR)/*
