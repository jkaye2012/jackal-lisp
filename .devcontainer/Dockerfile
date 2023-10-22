FROM ubuntu:22.04

RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install -y git \
  vim \
  curl \
  build-essential \
  sudo \
  tzdata \
  python3 \
  python3-pip \
  python-is-python3

RUN ln -sf /usr/share/zoneinfo/America/Denver /etc/localtime

ARG USERNAME=jkaye
ENV DEVFLOW_USERNAME=${USERNAME}
ARG USER_UID=1000
ARG USER_GID=1000
RUN groupadd --gid $USER_GID $USERNAME \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
  && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
  && chmod 0440 /etc/sudoers.d/$USERNAME
RUN usermod -s /bin/bash $USERNAME
ENV HOME=/home/jkaye
USER $USERNAME
SHELL ["/bin/bash", "-c"]

RUN sudo mkdir -p /opt/devflow/rust
RUN sudo chown $DEVFLOW_USERNAME /opt/devflow/rust
WORKDIR /opt/devflow/rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source "$HOME/.cargo/env"
ENV CARGO_HOME="$HOME/.cargo"
RUN sudo mkdir -p /opt/devflow/git
RUN sudo chown $DEVFLOW_USERNAME /opt/devflow/git
WORKDIR /opt/devflow/git

RUN python -m pip install mkdocs-material

WORKDIR $HOME
