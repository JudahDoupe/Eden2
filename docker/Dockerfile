# Use Node.js as the base image for serving static files
FROM node:18-alpine

# Install a simple HTTP server
RUN npm install -g http-server

# Set working directory
WORKDIR /app

# Copy the web assets
COPY web/ ./

# Expose port 8080
EXPOSE 8080

# Start the HTTP server
CMD ["http-server", ".", "-p", "8080", "-c-1", "--cors"]
