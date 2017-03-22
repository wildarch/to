#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Use this to change the target implementation
target=haskell

case "$target" in
    rust)
        TO="$DIR/rust/target/release/to"
        ;;
    lua)
        TO="$DIR/lua/main.lua"
        ;;
    python)
        TO="$DIR/python/main.py"
        ;;
    haskell)
        TO="$DIR/haskell/.stack-work/install/x86_64-linux/lts-8.0/8.0.2/bin/haskell-exe"
        ;;
    *)
        echo "Error! Unknown target language $target"
        #exit 1
        ;;
esac

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
        "--version" | "-v")
            $TO version
            ;;
        "")
            echo "Usage: to <dir>"
            echo "Tip: Use the tab key to autocomplete."
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
