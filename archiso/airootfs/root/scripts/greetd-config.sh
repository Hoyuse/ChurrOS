#!/usr/bin/env bash
set -e

USERNAME=$(awk -F: '$3>=1000 && $3<65534 {print $1; exit}' /etc/passwd)

if [ -z "$USERNAME" ]; then
    echo "ERROR: No regular user found on target system!" >&2
    exit 1
fi

cat > /etc/greetd/config.toml << EOF
[terminal]
vt = 1

[default_session]
command = "/usr/bin/niri --session"
user = "$USERNAME"
EOF

echo "greetd config written for user: $USERNAME"
