# To
Change directory to any of your favorite folders with a single short command, wherever your are.

# Usage
    cd ~/workspace # Directory containing your often accessed folder 'project'
    to -a # Adds the current directory to your favorites
    
    cd  ~/somewhere/else/entirely
    # Changes the current directory to ~/workspace/project
    to project
    # Or type:
    to proj
    # Then press TAB to autocomplete, or press enter for an "I'm feeling lucky"
    
    # List all favorite folders
    to -l
    # output:
    # [0]   /home/you/workspace
    
    # Remove the favorite at index 0
    to -r 0