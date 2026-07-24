# fix for screen readers
if grep -Fqa 'accessibility=' /proc/cmdline &> /dev/null; then
    setopt SINGLE_LINE_ZLE
fi

~/.automated_script.sh

# Start greetd — it conflicts with getty@tty1, so systemd will stop
# getty (killing this shell) and start greetd on tty1 automatically.
exec systemctl start greetd.service 2>/dev/null
