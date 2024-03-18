if exists('b:current_syntax')
    finish
endif

syntax keyword keywords fn return let if elif else while continue break for in
syntax keyword type uint int float str bool void
syntax keyword boolean true false

syntax keyword builtin print assert

syntax match comment '#.*$'
syntax region string start='"' end='"'
syntax region string start="'" end="'"

syntax match number '\<\d[0-9_]*\>'
syntax match number '\<\d[0-9_]*\.\d[0-9_]*\>'

syntax match func   '\(fn \)\@<=\w\+'
 
let b:current_syntax = 'chalcedony'
hi def link types    Type 
hi def link keywords Statement
hi def link builtin  Special
hi def link boolean  Constant
hi def link number   Constant
hi def link string   Constant
hi def link comment  Comment
hi def link func     Identifier 
