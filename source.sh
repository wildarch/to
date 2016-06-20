#!/bin/bash
TO="$HOME/.cargo/bin/to"

function _to_go {

	case "$1" in
		"--add" | "-a")
			$TO add $2
			;;
		"--dirs" | "-d" | "--list" | "-l")
			$TO dirs
			;;
		"--remove" | "-r")
			$TO remove $2
			;;
		*)
			cd "$($TO go $1)"
	esac
}
alias to="_to_go"

function _to_list {
	local cur
	local res

	COMPREPLY=()

	cur=${COMP_WORDS[COMP_CWORD]}
	res=$( $TO list $cur )
	if [ -n "$res" ]; then
		mapfile -t COMPREPLY < <($TO list $cur)
	fi
	compopt -o nospace
	return 0
}
complete -F _to_list to -o nospace
