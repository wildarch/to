#!/bin/bash
TO="/usr/bin/to"

function _to_go {
	cd "$($TO go $1)"
}
alias to="_to_go"

function _to_list {
	local cur
	local res
	
	COMPREPLY=()
	
	cur=${COMP_WORDS[COMP_CWORD]}
	res=$( $TO list $cur )
	if [ -n "$res" ]; then
		mapfile -t COMPREPLY < <(/usr/bin/to list $cur)
	fi
	compopt -o nospace
	return 0
}
complete -F _to_list to -o nospace
