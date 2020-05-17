Stuff to do, in no particular order

* Better parsing of attributes
  * Handle xml:lang
  * Be more strict about mandatory attributes (e.g. w/h on page) and surface errors to the caller
  * Use specific types (eg an URI type for FontUri instead of plain String)
  * More validation and surface errors to the caller
  * Handle attribute vs child element for things like RenderTransform
* Handle XPS resources (in page, global, etc)
* Rendering
  * Cairo renderer
  * raqote renderer?
* More parts (Fonts, Images, ...)
* More getters in the public API? (doc w/h?)
* Add tests
