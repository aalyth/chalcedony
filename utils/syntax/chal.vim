if exists('b:current_syntax')
    finish
endif

syntax keyword keywords fn return let if elif else while continue break try catch for in throw const import class
syntax keyword type uint int float str bool void exception
syntax keyword boolean true false

syntax keyword builtin print assert utoi ftoi itou ftou itof utof self

syntax match comment '#.*$'
syntax region string start='"' end='"'
syntax region string start="'" end="'"

syntax match number '\<\d[0-9_]*\>'
syntax match number '\<\d[0-9_]*\.\d[0-9_]*\>'

syntax match func '\(fn \)\@<=\w\+'
syntax match unsafe_func '\w\+!(\@='
syntax match method '\.\@<=\w\+(\@='

syntax match namespace '\w\+\(::\)\@='

let b:current_syntax = 'chalcedony'
hi def link types         Type 
hi def link keywords      Statement
hi def link builtin       SpecialChar
hi def link boolean       Constant
hi def link number        Constant
hi def link string        Constant
hi def link comment       Comment
hi def link func          Identifier 
hi def link unsafe_func   Exception
hi def link method        Function
hi def link namespace     Define 
