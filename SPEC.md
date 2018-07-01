# Settings
Settings are stored at $HOME/.config/to
## Format

    {
        "directories": [
            "dir1",
            "dir2"
        ]
    }
Directories must be absolute paths
    
## Loading
If the file at $HOME/.config/to does not exist:
1. Create path $HOME/.config/to
2. Create a settings file with no directories

If the file can not be parsed as valid JSON or does not adhere to the format, 
the implementation should output "Error: *$HOME*/.config/to is not valid" and exit immediately.
    
## Saving
Always save the settings file in **pretty printed** JSON.

# Commands
Excessive arguments may be safely ignored.
 
## list
    to list [query]
Return the names of all directories *dir* such that:
1. *dir* is a sub-directory of one of the directories in the settings file
2. The name of *dir* starts with *query*

It is possible that some of the directories in the settings file are no longer valid (i.e. were deleted), 
these should be ignored.

output:
* The names all directories satisfying (1) and (2), if any.

## go
    to go [query]
Returns the absolute path to the directory *dir* such that: 
1. constraints (1) and (2) of the list command are met
2. No *dir2* also satisfying the above constraint exists such that: 
    * The length of the name of *dir2* is shorter than *dir*, **or**
    * The name of *dir* is equal to the name of *dir2*, and the parent directory of *dir2* has a lower index than the parent directory of *dir*    

The requirements can for example be met by first getting all sub-directories of all directories in the settings file,
 filtering on names that start with *query* and then sorting with a **stable** sorting algorithm ascending based on name length. 
 Then return the first element of this list, if any.
 
output:
* "*dir*" if found
* "Error: no results found" if no *dir* exists


## add
    to add [dir]
Adds a directory to the settings file. 
If *dir* is not specified, add the current working directory.

output:
* Nothing if successful
* "Error: not a valid directory" if *dir* is not a valid absolute path

## dirs
    to dirs
List all directories in the settings file. Indexing may start at either 0 or 1, but must be consistent with indexing of the remove command.

output:
* For every directory a line with the index and the absolute path
* "Error: No directories in settings file" if there are not directories in the settings file

## remove
    to remove [index]
Remove the directory at *index* in the list "directories" of the settings file.

output:
* Nothing if successful
* "Error: *index* is not a valid index" if *index* is not an integer, or exceeds the list bounds

## version
    to version
Print "To: *lang* version", where *lang* is a short description of the programming language used to write this implementation.
