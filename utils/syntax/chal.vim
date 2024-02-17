if exists('b:current_syntax')
    finish
endif

syntax keyword keywords fn return let if elif else while continue break 
syntax keyword type uint int float str bool void
syntax keyword boolean true false

syntax match comment '#.*$'
syntax region string start='"' end='"'
syntax region string start="'" end="'"

syntax match number '\<\d\+\>'
syntax match number '\<\d\+\.\d\+\>'

syntax match func '\(fn \)\@<=\w\+'

let b:current_syntax = 'chalcedony'
hi def link types    Label
hi def link keywords Statement
hi def link boolean  Constant
hi def link number   Constant
hi def link string   Constant
hi def link comment  Comment
hi def link func     Blue 
