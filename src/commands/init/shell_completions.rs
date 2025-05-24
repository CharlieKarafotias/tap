pub(super) const ZSH_COMPLETION: &str = r#"#compdef tap

# Fetch parent entities dynamically by running `tap -s`. Then:
# - skip the first line
# - remove leading and trailing whitespace
# - remove empty lines
local -a parents
parents=("${(@f)$(tap -s | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d')}")

# TODO: fetch dynamically using --help command (temp workaround)
local -a commands
commands=(
  '(-a --add)'{-a,--add}'[Add a new link]'
  '(-d --delete)'{-d,--delete}'[Deletes a link]'
  '(-s --show)'{-s,--show}'[Shows links]'
  '(-u --upsert)'{-u,--upsert}'[Create/update a link]'
  '(-i --init)'{-i,--init}'[Setup Tap and shell completions ]'
  '(--import)'--import'[Imports links from file]'
  '(--export)'--export'[Exports links to file]'
  '(--tui)'--tui'[Launch the interactive UI]'
  '(--update)'--update'[Update Tap to the latest version]'
  '(--help)'--help'[Display this help message]'
  '(-v --version)'{-v,--version}'[Show tap version]'
)

# Extract just the command names (without the options) for comparison
local -a command_options
command_options=(
  '-a'
  '--add'
  '-d'
  '--delete'
  '-s'
  '--show'
  '-u'
  '--upsert'
  '-i'
  '--init'
  '--import'
  '--export'
  '--tui'
  '--update'
  '--help'
  '-v'
  '--version'
)

_arguments \
  '1:parent entity:->parent' \
  '2:command:->command' \
  '*::args:->args'

case $state in
    parent)
        if [[ "$parents" ]]; then
            _values 'Parent entities' $parents $commands
        else
            _values 'No parent entities available' $commands
        fi
    ;;
    command)
        local selected_parent=$words[2]
        
        # Check if the selected word is a command or a parent entity
        if (($command_options[(Ie)$selected_parent])); then
            # If it's a command option, provide command-specific completions
            case $selected_parent in
                -a|--add)
                    _values 'Add options' "parent" "link" "value"
                    ;;
                -d|--delete)
                    _values 'Delete options' "parent" "link"
                    ;;
            esac
        else
            print "DEBUG: Selected parent entity: $selected_parent"
            local selected_links
            selected_links=($(tap -s $selected_parent | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d'))
            _values 'Links' $selected_links
        fi
    ;;
esac
"#;
