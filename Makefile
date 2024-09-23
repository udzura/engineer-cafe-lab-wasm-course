docs/index.html: docs/index.md
	cd docs && marp index.md --theme default --html

00_installday/index.html: 00_installday/index.md
	cd 00_installday && marp index.md --theme ./ecl.css --html
