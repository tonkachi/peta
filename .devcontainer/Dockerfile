# See here for image contents: https://github.com/microsoft/vscode-dev-containers/blob/v0.202.5/containers/codespaces-linux/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/universal:1-linux

USER root

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends build-essential make curl bash zsh fish direnv && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# rust-components
RUN rustup component add rust-src && \
    rustup component add rust-analysis && \
    rustup component add rls

# diesel
RUN apt-get update && apt-get -y install --no-install-recommends libpq-dev && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/* && \
    cargo install diesel_cli --no-default-features --features postgres

# grpcurl
RUN go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest

# yarn
RUN npm install -g yarn

# typescript
RUN npm install -g typescript

# skaffold
RUN curl -Lo /usr/local/bin/skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
    chmod +x /usr/local/bin/skaffold

# kind
RUN curl -Lo /usr/local/bin/kind https://kind.sigs.k8s.io/dl/v0.11.1/kind-linux-amd64 && \
    chmod +x /usr/local/bin/kind

# kubectl
RUN curl -Lo /usr/local/bin/kubectl https://storage.googleapis.com/kubernetes-release/release/$(curl -s https://storage.googleapis.com/kubernetes-release/release/stable.txt)/bin/linux/amd64/kubectl && \
    chmod +x /usr/local/bin/kubectl
