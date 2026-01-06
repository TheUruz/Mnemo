MNEMO_FIRST_RUN=true
mnemo_hook() {
    local mnemo_path="mnemo"
    command -v "$mnemo_path" >/dev/null 2>&1 || return 1
    if [[ "$MNEMO_FIRST_RUN" == true ]]; then
        MNEMO_FIRST_RUN=false
        return
    fi

    local last_cmd
    last_cmd=$(fc -ln -1)
    last_cmd="${last_cmd#"${last_cmd%%[![:space:]]*}"}"
    [[ -n "$last_cmd" ]] && "$mnemo_path" -H "$last_cmd"
}

autoload -Uz add-zsh-hook
add-zsh-hook precmd mnemo_hook
