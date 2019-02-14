FROM rust:1.32

LABEL maintainer="Murat Eksi"

# Configure required environment
ENV MIX_ENV dev
ENV WORKSPACE /rust-workspace

# Create and set home directory
RUN mkdir $WORKSPACE
WORKDIR $WORKSPACE

#RUN cargo install --path .

# Copy all application files
COPY . $WORKSPACE