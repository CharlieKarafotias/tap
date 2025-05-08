pub(super) const ZSH_COMPLETION: &str = r#"#compdef tap

# Fetch parent entities dynamically by running `tap -s`. Then:
# - skip the first line
# - remove leading and trailing whitespace
# - remove empty lines
local -a parents
parents=("${(@f)$(tap -s | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d')}")

_arguments \
  '1:parent entity:->parent' \
  '*::args:->args'

# Capture and parse commands from the tap output for autocompletion
# TODO: basically need to finish this where if -char, OR -- pattern, then add to commands OTHERWISE if <Parent> then use parents local OTHERWISE here
local -a commands
commands=( ${(f)$(tap | sed -n -e '/^Commands:/,/^Usage:/p | sed -e '1d;$d' -e 's/  */ /g' -e 's/^ *//;s/ *$//')} ) }

_arguments \
  '1:tap command:->cmds' \
  '*::args:->args'

case $state in
    cmds)
        _describe -t commands 'tap command' commands
        ret=0
        ;;
    parent)
        if [[ "$parents" ]]; then
            _values 'Parent entities' $parents
        fi
    ;;
esac
"#;
