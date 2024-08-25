FROM ubuntu:24.10
# Update default packages
RUN apt-get update
SHELL ["/bin/bash", "-c"]
# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl
# Update new packages
RUN apt-get update
# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN source $HOME/.cargo/env
# Get npm
RUN apt-get install -y npm
RUN npm install -g npm@latest
# Install wasm-pack
RUN cargo install wasm-pack
# Create app dir
RUN mkdir -p app
# Copy files
COPY . /app
WORKDIR /app
RUN cargo add wasm-bindgen
RUN wasm-pack build --target bundler
# Serve
WORKDIR /app/site
# Install webpack & package deps
RUN npm i ../pkg
RUN npm i -D webpack@5 webpack-cli@5 webpack-dev-server@4 copy-webpack-plugin@11
RUN npm i fabric
EXPOSE 8080
ENTRYPOINT [ "npm", "run", "serve" ]