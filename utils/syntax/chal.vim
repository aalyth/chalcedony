if exists("b:current_syntax")
    finish
endif

syn keyword keywords fn return let if elif else while pass
syn keyword type uint int float str bool void
syn keyword boolean true false

syn match comment '#.*$'
syn region string start='"' end='"'
syn region string start="'" end="'"

syn match number '\d\+'
syn match number ' -\d\+'
syn match number '\d\+\.\d\+'
syn match number ' -\d\+\.\d\+'

let b:current_syntax = 'chalcedony'
hi def link types    Type
hi def link keywords Statement
hi def link boolean  Constant
hi def link number   Constant
hi def link string   Constant
hi def link comment  Comment
