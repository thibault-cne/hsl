; Identifier

(identifier) @variable

; External functions

(extern_function
  name: (identifier) @function)

; Function calls

(function_call
  name: (identifier) @function)

; Function declaration

(function_definition
  name: (identifier) @function)

; Variadics

(variadic
  count: (unsigned_integer) @constant.numeric.integer)

; Literals

(integer) @constant.builtin
(bool) @constant.builtin

(char) @string
(string) @string

; Comments

(line_comment) @comment

; Keywords

[
  "Execute order"
  "Order executed"
  "Starfield"
] @keyword

[
  "Hypersignal"
  "Jamsignal"
] @keyword.control.import

[
  
  "I am a big deal in the resistance."
  "The force is strong with this one."
  "That's one hell of a pilot."
  "I am your father."
  "Judge me by my size, do you ?"
] @keyword.storage.type

[
  
  "A long time ago in a"
  "far, far away..."
  "May the force be with you."
] @keyword.function

; Booleans

[
  "From a certain point of view."
  "That's impossible!"
] @constant.builtin.boolean
