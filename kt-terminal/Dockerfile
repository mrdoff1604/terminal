# Multi-stage Dockerfile for kt-terminal

# Stage 1: Build the application using Gradle
FROM openjdk:21-jdk-slim AS builder

# Set working directory
WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash \
    && rm -rf /var/lib/apt/lists/*

# Copy Gradle wrapper files first to leverage Docker cache
COPY gradle/ gradle/
COPY gradlew ./
COPY gradlew.bat ./
COPY settings.gradle.kts ./
COPY build.gradle.kts ./

# Make Gradle wrapper executable
RUN chmod +x ./gradlew

# Copy source code
COPY src/ src/

# Build the fat JAR using shadowJar
RUN ./gradlew shadowJar

# Stage 2: Create lightweight runtime image
FROM openjdk:21-jre-slim

# Set working directory
WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash \
    && rm -rf /var/lib/apt/lists/*

# Copy the built JAR from the builder stage
COPY --from=builder /app/build/libs/kt-terminal-*-fat.jar app.jar

# Expose the application port (as configured in application.conf)
EXPOSE 8080

# Set environment variables (optional, can be overridden at runtime)
ENV PORT=8080

# Create a non-root user for security
RUN useradd -m -u 1000 terminaluser
USER terminaluser

# Set entry point to run the application
ENTRYPOINT ["java", "-jar", "app.jar"]
