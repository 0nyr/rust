# Markdown serving

The objective is to build a simple web server that serves static html files. But when launched, the server needs to compile .md files to .html.

### Useful links

##### guides

[Using rust to build a blog site](https://patshaughnessy.net/2019/9/4/using-rust-to-build-a-blog-site)

##### projects

[pat shaughnessy blog generator](https://github.com/patshaughnessy/patshaughnessy.github.io)

[markdown2html-converter](https://github.com/magiclen/markdown2html-converter/blob/master/example.md)

### Objective

1. Deploy and compile to html

In this step, rust transforms .md files into bodies of html.

2. Deploy and compile to Handlebars

The idea to transform .md files into bodies of Handlebars and serve that using Rocket
