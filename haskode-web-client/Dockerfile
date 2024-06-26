
# Stage 1: Build the application
FROM node:18 AS build

# Set the working directory
WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Build the application
RUN npm run build

# Stage 2: Run the application
FROM node:18

# Set the working directory
WORKDIR /app

# Copy only the necessary files from the build stage
COPY --from=build /app ./

# Install only production dependencies
RUN npm install --production

# Expose port 3000
EXPOSE 3000

# Start the SvelteKit server
CMD ["node", "build"]

