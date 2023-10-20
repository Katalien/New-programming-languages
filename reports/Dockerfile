# Use the official Rust image as the base image
FROM rust:latest

# Set environment variables
ENV PROJECT_NAME="hello_world"
ENV BOOT_NAME="hello_world"

# Set the working directory inside the container
WORKDIR /app

# Create a shell script to run your Rust application
RUN echo "#!/bin/sh" > run_app.sh && \
    echo "exec ./target/debug/\$PROJECT_NAME/" >> run_app.sh && \
    chmod +x run_app.sh

# Copy the Cargo.toml and Cargo.lock files to the container
COPY $PROJECT_NAME/Cargo.toml $PROJECT_NAME/Cargo.lock ./

# Create an empty dummy project to pre-fetch dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build && rm -r src

# Copy the entire project directory into the container
COPY $PROJECT_NAME ./

# Build the Rust project
RUN cargo build

# Define the command to run your project using the shell script
CMD ["./run_app.sh"]
