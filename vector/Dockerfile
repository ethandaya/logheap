# Use the official Vector image as the base
FROM timberio/vector:latest-alpine

# Copy the Vector configuration file from the local filesystem
COPY vector.toml /etc/vector/vector.toml

# Expose the required ports
EXPOSE 9000 9001 3030

#ENV VECTOR_LOG="debug"

# Start the Vector service with the provided configuration
CMD ["-c", "/etc/vector/vector.toml"]
