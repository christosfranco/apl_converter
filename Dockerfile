# Use the Rust official image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files into the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code into the container
COPY src ./src

# Copy the source code into the container
# COPY APL_examples ./APL_examples


# Build the Rust application
RUN cargo build --release

# Expose any necessary ports
# EXPOSE 8080

# Command to run the application
CMD ["./target/release/apl_converter"]


#
#sudo docker run -d -p 8080:8080 -v "$(pwd)/APL_examples:/APL_examples" apl_converter ./target/release/apl_converter -f /APL_examples/matrix.apl && sudo docker logs 003619a90a0b35424f2c9e03c45494f014c7c045a7e1e90246f7241610d96c0e