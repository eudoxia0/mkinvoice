PDF := invoice.pdf
PNG := invoice.png

all: $(PDF) $(PNG)

$(PDF): invoice.toml
	mkinvoice $< $@

$(PNG): $(PDF)
	magick -density 300 -quality 90 $< $@

clean:
	rm -f $(PDF) $(PNG)
