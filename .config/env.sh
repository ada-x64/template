export SSH_IP=$(echo $SSH_CLIENT | cut -d' ' -f1)
export SSH_PORT=$(echo $SSH_CLIENT | cut -d' ' -f3)
export PATH_HASH=$(echo $PWD | cksum | cut -d' ' -f1)
export SSH_OUT="~/.local/share/rsync/$PATH_HASH"
export CARGO_TERM_COLOR="always"

if [[ -n "$SSH_IP" ]]; then
   export FEATURES="dev"
else
    export FEATURES="dev dylib"
fi

if [[ -n "$CI" ]]; then
    export MISE_ENV="ci"
elif [[ -n "$SSH_IP" ]]; then
    export MISE_ENV="ssh"
fi
