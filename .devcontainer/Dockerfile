# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.194.3/containers/debian/.devcontainer/base.Dockerfile

# [Choice] Debian version: bullseye, buster, stretch
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

ENV CARGO_BUILD_TARGET_DIR=/cache/target
RUN mkdir -p $CARGO_BUILD_TARGET_DIR

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends build-essential make curl bash zsh fish && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# docker-cli
RUN apt-get update && \
    apt-get -y install --no-install-recommends apt-transport-https ca-certificates curl gnupg lsb-release && \
    curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null && \
    apt-get update && apt-get -y install docker-ce-cli && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# nodejs and npm
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash - && apt install -y nodejs && \
    apt-get clean -y && rm -rf /var/lib/apt/lists/*

# golang
ENV GOPATH /go
ENV GOROOT /usr/local/go
ENV PATH $PATH:$GOROOT/bin:$GOPATH/bin
RUN curl -sLo go.tar.gz https://golang.org/dl/go1.17.2.linux-amd64.tar.gz && \
    rm -rf /usr/local/go && tar -C /usr/local -xzf go.tar.gz && \
    mkdir -p $GOPATH

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

# grpcurl
RUN go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
