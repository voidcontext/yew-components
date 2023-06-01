#!/bin/bash

echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf
echo "trusted-substituters = file:///var/lib/woodpecker-agent/nix-store" >> /etc/nix/nix.conf
echo "extra-trusted-public-keys = $(cat /var/lib/woodpecker-agent/nix-store/cache-pub-key.pem)" >> /etc/nix/nix.conf
echo "extra-substituters = file:///var/lib/woodpecker-agent/nix-store" >> /etc/nix/nix.conf
