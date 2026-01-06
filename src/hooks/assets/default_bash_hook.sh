MNEMO_FIRST_RUN=true
mnemo_hook() {
    local mnemo_path="mnemo"
    command -v "$mnemo_path" >/dev/null 2>&1 || return 1

    if [ "$MNEMO_FIRST_RUN" = true ]; then
        MNEMO_FIRST_RUN=false
        return
    fi

    local cmd
    cmd=$(history 1 | sed 's/^[ ]*[0-9]\+[ ]*//')
    [ -n "$cmd" ] && "$mnemo_path" -H "$cmd"
}

PROMPT_COMMAND="mnemo_hook${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
