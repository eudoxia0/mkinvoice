# mkinvoice

[![test](https://github.com/eudoxia0/mkinvoice/actions/workflows/test.yaml/badge.svg)](https://github.com/eudoxia0/mkinvoice/actions/workflows/test.yaml)

Script to create beautiful PDF invoices for my consulting practice.

Requires [Chromium][chrom]: invoices are rendered to HTML and then to PDF using
headless Chromium's print-to-PDF function.

[chrom]: https://en.wikipedia.org/wiki/Chromium_(web_browser)

## Usage

See the `example/` directory for an example [`invoice.toml`][invoice].

[invoice]: https://github.com/eudoxia0/mkinvoice/blob/master/example/invoice.toml

```
$ mkinvoice invoice.toml invoice.pdf
```

## License

© 2026 by [Fernando Borretti][fb]. Released under the [Apache 2.0][apache2] license.

[fb]: https://borretti.me/
[apache2]: https://www.apache.org/licenses/LICENSE-2.0
