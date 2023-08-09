FROM rust:1.71 as BUILD

SHELL ["/bin/bash", "-c"]

# Install just
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Setup fnm
RUN curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir /usr/local/bin && \
    echo 'eval "`fnm env`"' > ~/.bashrc && \
    echo 'eval "$(fnm env --use-on-cd)"' > ~/.bashrc

WORKDIR /app

RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcpu.zip -O libtorch.zip
RUN unzip -o libtorch.zip
ENV LIBTORCH /app/libtorch
ENV LD_LIBRARY_PATH /app/libtorch/lib:$LD_LIBRARY_PATH

COPY . .

# Setup fnm
RUN fnm install && source ~/.bashrc && fnm use

RUN source ~/.bashrc && just package

# Runtime image
FROM debian:bullseye-slim as RUNTIME

RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates wget unzip libgomp1

WORKDIR /app

RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.0.1%2Bcpu.zip -O libtorch.zip
RUN unzip -o libtorch.zip
ENV LIBTORCH /app/libtorch
ENV LD_LIBRARY_PATH /app/libtorch/lib:$LD_LIBRARY_PATH

WORKDIR /app/build-app

COPY --from=BUILD /app/build-app /app/build-app

EXPOSE 8080

ENV RUSTBERT_CACHE=/data/rustbert_cache
ENV RUST_LOG=info

ENTRYPOINT [ "./backend-server" ]
