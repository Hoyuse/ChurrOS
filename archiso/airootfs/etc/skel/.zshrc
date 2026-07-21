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

# Colors
autoload -Uz colors
colors

# Starship prompt
eval "$(starship init zsh)"
