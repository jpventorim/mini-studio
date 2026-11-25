# Changelog

## [0.2.0](https://github.com/jpventorim/mini-studio/compare/v0.1.0...v0.2.0) (2026-11-25)


### Features

* **app:** use Rusty Bob desktop icons ([584876c](https://github.com/jpventorim/mini-studio/commit/584876c8c7c6065d67358f3f17c608c42b0ea8b2))
* **cli:** add image borders for layout validation ([f132f7a](https://github.com/jpventorim/mini-studio/commit/f132f7a5fa1f49c0e2a74925ae6a22905ab31a69))
* **cli:** enable rotation and default to configurable 3px borders ([570942f](https://github.com/jpventorim/mini-studio/commit/570942f23372ceeb31f5011876c06ba0aa15ede8))
* **image-manipulation:** add optional per-image copy numbering ([627bedf](https://github.com/jpventorim/mini-studio/commit/627bedfa9e173a973a3d79516e8ac253328ea4b2))
* **image-manipulation:** generate foldable standees with position presets ([d41a0a4](https://github.com/jpventorim/mini-studio/commit/d41a0a43026c711e47abf4e7df953ff6edd0ed94))
* pack images into multi-page PDFs with library and CLI ([b336619](https://github.com/jpventorim/mini-studio/commit/b336619b1881aa63e44e6554d386b9724794892d))
* **packing:** default image spacing to zero for cutting ([379fd01](https://github.com/jpventorim/mini-studio/commit/379fd0189e7a5fd2538fbdd731aac9cf1ffe6cbd))
* **packing:** default paper margins to zero ([e59fdfc](https://github.com/jpventorim/mini-studio/commit/e59fdfc65bd242b91909f97a2878a3fb115cce66))
* **packing:** default to configurable 3mm paper margins ([2f2e98e](https://github.com/jpventorim/mini-studio/commit/2f2e98e02172c0f8fca2472aae9080980cc063f4))
* **print:** add A2 and A3 paper sizes and update layout defaults ([2da9c06](https://github.com/jpventorim/mini-studio/commit/2da9c069591baada90aaf4846095392e26be5758))
* **print:** add creature size categories and final dimensions ([b572492](https://github.com/jpventorim/mini-studio/commit/b5724921a51335ee6ac301907cdbd0794b7bca85))
* **print:** improve color management and preserve 16-bit PDF artwork ([19d0d15](https://github.com/jpventorim/mini-studio/commit/19d0d15d0b3dcc8e1f5cd5b7f31d6656161227dd))
* **print:** prepare grid-sized standees before packing ([ae9c876](https://github.com/jpventorim/mini-studio/commit/ae9c876cf8018a6c436c6e777a4da828648a7d82))
* **print:** save export settings and request actual-size printing ([dff5cfd](https://github.com/jpventorim/mini-studio/commit/dff5cfd4998b4cd5f1b509a98c980618181ea3f5))
* **print:** step margin and spacing by 0.5 mm with arrow keys ([c408a4e](https://github.com/jpventorim/mini-studio/commit/c408a4e4d65d9edb9e110af2506061376bfb10d0))
* **ui:** add Base UI design system with Panda CSS and Storybook ([b53ef3c](https://github.com/jpventorim/mini-studio/commit/b53ef3c9871eb73f26c198256d55155b6788e2f5))
* **ui:** refine components with medieval materials and physical controls ([edc84aa](https://github.com/jpventorim/mini-studio/commit/edc84aa0e615dab349ea07786baf130670677fc4))
* **ui:** unify material surfaces and add light and dark themes ([0346ef1](https://github.com/jpventorim/mini-studio/commit/0346ef1c29962d057e91310f763b75ac39adf0de))


### Bug Fixes

* **image-manipulation:** fit numbers in quarter-size base sections ([409e86d](https://github.com/jpventorim/mini-studio/commit/409e86df7c44974422c079d3ecfac9b628e86283))
* **packing:** minimize page count and fill earlier pages densely ([f43839c](https://github.com/jpventorim/mini-studio/commit/f43839cc700acd78e34f5bcaf64370c7d0819666))


### Performance Improvements

* **print:** cache PDF bytes for unchanged exports ([53df6b1](https://github.com/jpventorim/mini-studio/commit/53df6b1e4df053d06c08d0e4331cf74cba3e2b62))
* **print:** reuse prepared standees and streamline previews ([3f973be](https://github.com/jpventorim/mini-studio/commit/3f973be9c63f31ce8cc6e10e2b0f6c2fd6c6f2ee))
