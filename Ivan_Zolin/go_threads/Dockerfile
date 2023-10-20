# Use an official Go runtime as a parent image
FROM golang:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the local source files to the container's working directory
COPY . .

# Build the Go application
RUN go build -o go_threads

# Expose the port the application will run on
EXPOSE 8080

# Command to run the application
CMD ["./go_threads"]
