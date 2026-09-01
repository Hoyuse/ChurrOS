# ChurrOS ZSH Config

# History
HISTSIZE=10000
SAVEHIST=10000
HISTFILE=~/.zsh_history
setopt HIST_IGNORE_ALL_DUPS
setopt SHARE_HISTORY

# Completion
autoload -Uz compinit
compinit

# ============================================================
# NORMAL WORD MOVEMENT

# Ctrl + Left
power-left() {
    REGION_ACTIVE=0
    zle backward-word
}

# Ctrl + Right
power-right() {
    REGION_ACTIVE=0
    zle forward-word
}

zle -N power-left
zle -N power-right

# ============================================================
# DELETE WORD

# Ctrl + Backspace
power-backspace() {
    if (( REGION_ACTIVE )); then
        zle kill-region
    else
        zle backward-kill-word
    fi
}

# Ctrl + Delete
power-delete() {
    if (( REGION_ACTIVE )); then
        zle kill-region
    else
        zle kill-word
    fi
}

zle -N power-backspace
zle -N power-delete

# ==============================================================
# KEY BINDINGS

# Normal word movement
bindkey $'\e[1;5D' power-left
bindkey $'\e[1;5C' power-right

# Delete words
bindkey $'\e[99~' power-backspace
bindkey $'\e[3;5~' power-delete

# --- Prompt (starship style) ---
setopt PROMPT_SUBST EXTENDED_GLOB

autoload -Uz vcs_info add-zsh-hook

zstyle ':vcs_info:*' enable git
zstyle ':vcs_info:git:*' check-for-changes true
zstyle ':vcs_info:git:*' stagedstr '%F{#B19C8D} ✓%f'
zstyle ':vcs_info:git:*' unstagedstr '%F{#BC8960} ✎%f'
zstyle ':vcs_info:git:*' formats ' [ ⬡ %b%u%c ]'

function _churros_prompt() {
    vcs_info
    local last_exit=$?

    local left="%F{#E9D9CF}[ ⬡ churros ]%f %F{#E9D9CF}[ %~ ]%f%F{#D8C7B4}${vcs_info_msg_0_}%f"

    local right=""
    if (( ${CMD_DURATION:-0} > 3000 )); then
        right="%F{#CEAE98}[ ⏱ $(( CMD_DURATION / 1000 ))s ]%f "
    fi
    right+="%F{#cbc5c4}[ ⏣ %* ]%f"

    local left_text=$(print -Pn "$left")
    local right_text=$(print -Pn "$right")
    left_text=${left_text//$'\e'\[[0-9;]#m/}
    right_text=${right_text//$'\e'\[[0-9;]#m/}

    local pad=$(( COLUMNS - ${#left_text} - ${#right_text} ))

    local dots=''
    if (( pad > 0 )); then
        local _dots='··································································································································································································································'
        dots=${_dots:0:$pad}
    fi

    local char_color='#E9D9CF'
    (( last_exit != 0 )) && char_color='#BC8960'

    PROMPT="${left}%F{#cbc5c4}${dots}%f${right}"$'\n'"%F{${char_color}}❯%f "
}
add-zsh-hook precmd _churros_prompt

if [ -z "$FASTFETCH_SHOWN" ] && command -v fastfetch >/dev/null 2>&1; then
    fastfetch
    export FASTFETCH_SHOWN=1
fi
