# POSIX interactive ENV bootstrap. Native login profiles have already run.
_khor_original_env=${KHOR_ORIG_ENV-}
if [ -n "${KHOR_ORIG_ENV+set}" ]; then
  ENV=$KHOR_ORIG_ENV
  export ENV
else
  unset ENV
fi
unset KHOR_ORIG_ENV
if [ -n "$_khor_original_env" ] && [ -r "$_khor_original_env" ]; then
  . "$_khor_original_env"
fi
unset _khor_original_env
# POSIX shells have no universal preexec hook. This only proves that their
# native interactive startup has completed, and marks subsequent prompts.
PS1="${PS1-\$ }$(printf '\033]133;B\007')"
