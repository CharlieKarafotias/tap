pub(super) const ZSH_COMPLETION: &str = r#"#compdef tap

# zsh completion wrapper for tap
#
# The recommended way to install this script is to make a copy of it as a file named '_tap'
# in a directory in your $fpath. For example:
#
#   mkdir -p ~/.zsh/
#   tap init zsh > ~/.zsh/_tap
#
# Then add the following line to your ~/.zshrc file:
#
#   fpath=(~/.zsh/ $fpath)
#
# You will also need to add the following line to your ~/.zshrc file:
#
#   autoload -Uz compinit && compinit
#
# For more information, see:
# https://zsh.sourceforge.io/Doc/Release/Completion-System.html#Initialization
#
# Alternatively, you can run the following tap command to automatically setup shell completions:
#
#   tap init --auto

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
  '3:commandargs:->commandargs' \
  '*::args:->args'

case $state in
    parent)
        if [[ -n "words[2]" && "${words[2]:0:1}" != "-" ]]; then
            _values 'Parent entities' $parents
        else
            _values 'Commands' $commands
        fi
    ;;
    command)
        local selected_parent=$words[2]
        
        # Check if the selected word is a command or a parent entity
        if (($command_options[(Ie)$selected_parent])); then
            # Handle command descriptions
            case $selected_parent in
                -a|--add)
                    _values 'Add options' 'here' $parents
                    ;;
                -d|--delete)
                    _values 'Delete options' 'here' $parents
                    ;;
                -s|--show)
                    _values 'Show options' 'here' $parents
                    ;;
                -u|--upsert)
                    _values 'Upsert options' 'here' $parents
                    ;;
                --import)
                    _values 'Import options' 'Chrome' 'Edge' 'Firefox' 'Opera' 'Safari' 'Tap'
                    ;;
                --export)
                    _values 'Export options' 'Chrome' 'Edge' 'Firefox' 'Opera' 'Safari' 'Tap'
                    ;;
            esac
        else
            local selected_links
            selected_links=($(tap -s $selected_parent | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d'))
            _values 'Links' $selected_links
        fi
    ;;
    commandargs)
        local command=$words[2]
        local subcommand=$words[3]
        if [[ $command == "--import" || $command == "--export" ]]; then
                _files
        fi
        if [[ $command == "-d" || $command == "--delete" || $command == "-s" || $command == "--show" || $command == "-u" || $command == "--upsert" ]]; then
            local selected_links
            selected_links=($(tap -s $subcommand | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d'))
            _values 'Links' $selected_links
        fi
    ;;
esac
"#;
