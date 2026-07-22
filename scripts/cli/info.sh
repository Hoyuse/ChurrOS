#!/usr/bin/env bash

echo "===================================="
echo "          ChurrOS CLI"
echo "===================================="
echo

echo "Project      : ChurrOS"
echo "Version      : 0.1.0 Alpha"
echo "Branch       : $(git branch --show-current)"
echo "Architecture : $(uname -m)"
echo "Kernel        : $(uname -r)"
echo

echo "Directories"
echo "-----------"

echo "Profile : archiso/"
echo "Output  : out/"
echo "Work    : work/"
echo "Scripts : scripts/"