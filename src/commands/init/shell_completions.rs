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

case $state in
  parent)
    _values 'Parent entities' $parents
    ;;
esac
"#;
