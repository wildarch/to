# put this file in /etc/bash_completion.d/ 
# daandegraaf9@gmail.com

_to()
{
    local cur
    # Pointer to current completion word.
    # By convention, it's named "cur" but this isn't strictly necessary.

    COMPREPLY=()   # Array variable storing the possible completions.
    cur=${COMP_WORDS[COMP_CWORD]}
    
    COMPREPLY=( $( compgen -W 'Hello world this is a test' ) )

    return 0
}

complete -F _to $filenames ./to