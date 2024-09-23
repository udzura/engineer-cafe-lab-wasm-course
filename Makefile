docs/index.html:
	cd docs && marp index.md --theme default --html

00_installday/index.html:
	cd 00_installday && marp index.md --theme ./ecl.css --html
