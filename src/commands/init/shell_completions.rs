pub(super) const ZSH_COMPLETION: &str = r#"#compdef tap

# Fetch parent entities dynamically by running `tap -s`. Then:
# - skip the first line
# - remove leading and trailing whitespace
# - remove empty lines
local -a parents
parents=("${(@f)$(tap -s | tail -n +2 | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e '/^$/d')}")

# TODO: fetch dynamically (need to update so commands without groups for ex: export do not wrap in {}. Also need to figure out why the commands arent parsing correctly in ZSH)
# commands=("${(@f)$(
#   tap --help |
#   sed -n '/^Commands:/,$p' |
#   sed '1d' |
#   grep '^[[:space:]]*-' |
#   awk '{ 
#     sub(/^[[:space:]]*/, "", $0);
#     split($0, parts, /[[:space:]]{2,}/);
#     opts_raw = parts[1];
#     if (length(parts) > 1 && parts[length(parts)] == "") {
#         desc = parts[length(parts) - 1];
#     } else {
#         desc = parts[length(parts)];
#     }
#     sub(/, /, ",", opts_raw);
#     opts_spaced = opts_raw;
#     sub(/,/, " ", opts_spaced);
#     q="\047";
#     printf "%s(%s)%s{%s}%s[%s]%s\n", q, opts_spaced, q, opts_raw, q, desc, q;
#   }'
# )}")

# BELOW is temporary while i figure out how to dynamically fetch commands
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

_arguments \
  '1:parent entity:->parent' \
  '*::args:->args'

case $state in
    parent)
        if [[ "$parents" ]]; then
            _values 'Parent entities' $parents $commands
        else
            _values 'No parent entities available' $commands
        fi
    ;;
esac
"#;
