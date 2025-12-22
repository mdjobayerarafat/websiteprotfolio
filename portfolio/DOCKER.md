# Docker Deployment Guide for Portfolio

This guide explains how to deploy your Portfolio application using Docker on a VPS.

## 📋 Prerequisites

- A VPS with Ubuntu 20.04+ (or any Linux distribution)
- Docker and Docker Compose installed
- Domain name (optional, for SSL)

## 🚀 Quick Start (Development)

```bash
# Clone your repository to the VPS
git clone <your-repo-url> portfolio
cd portfolio

# Make deploy script executable
chmod +x deploy.sh

# Build and start the application
./deploy.sh dev
```

Access your portfolio at `http://your-server-ip:8080`

## 🏭 Production Deployment

### Step 1: Prepare the Server

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo apt install docker-compose-plugin -y

# Add your user to docker group
sudo usermod -aG docker $USER
newgrp docker
```

### Step 2: Clone and Configure

```bash
# Clone the repository
git clone <your-repo-url> portfolio
cd portfolio

# Make deploy script executable
chmod +x deploy.sh
```

### Step 3: Configure Domain (Optional)

Edit `nginx/nginx.conf` and replace `yourdomain.com` with your actual domain:

```bash
nano nginx/nginx.conf
```

### Step 4: Deploy

```bash
# For development (HTTP only)
./deploy.sh dev

# For production with Nginx
./deploy.sh prod
```

## 🔒 SSL/HTTPS Setup with Let's Encrypt

### Step 1: Initial HTTP Setup

First, start without SSL to verify the setup:

```bash
./deploy.sh prod
```

### Step 2: Obtain SSL Certificate

```bash
# Create directories
mkdir -p certbot/conf certbot/www

# Get initial certificate
docker run -it --rm \
  -v $(pwd)/certbot/conf:/etc/letsencrypt \
  -v $(pwd)/certbot/www:/var/www/certbot \
  certbot/certbot certonly \
  --webroot \
  --webroot-path=/var/www/certbot \
  -d yourdomain.com \
  -d www.yourdomain.com \
  --email your@email.com \
  --agree-tos \
  --no-eff-email
```

### Step 3: Enable HTTPS in Nginx

Edit `nginx/nginx.conf`:
1. Uncomment the HTTPS server block
2. Update the domain names
3. Comment out or remove the HTTP server block (keep only the redirect)

```bash
nano nginx/nginx.conf
```

### Step 4: Restart Services

```bash
./deploy.sh prod
```

### Step 5: Auto-Renewal (Optional)

Uncomment the certbot service in `docker-compose.prod.yml` for automatic certificate renewal.

## 📁 File Structure

```
portfolio/
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # Development compose file
├── docker-compose.prod.yml # Production with Nginx
├── deploy.sh              # Deployment script
├── .dockerignore          # Files to exclude from Docker
├── .env.production        # Production environment template
└── nginx/
    └── nginx.conf         # Nginx configuration
```

## 🔧 Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |
| `DATABASE_URL` | `/app/data/portfolio.db` | SQLite database path |
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `8080` | Server port |

## 📊 Useful Commands

```bash
# View logs
./deploy.sh logs

# Stop all containers
./deploy.sh stop

# Rebuild and restart
./deploy.sh dev  # or prod

# Access container shell
docker exec -it portfolio-app /bin/sh

# View container stats
docker stats

# Backup database
docker cp portfolio-app:/app/data/portfolio.db ./backup.db

# Restore database
docker cp ./backup.db portfolio-app:/app/data/portfolio.db
docker restart portfolio-app
```

## 🗄️ Data Persistence

The SQLite database is stored in a Docker volume (`portfolio_data`). Your data persists even when containers are restarted or rebuilt.

To backup your data:
```bash
docker run --rm -v portfolio_data:/data -v $(pwd):/backup alpine \
  tar cvf /backup/portfolio-backup.tar /data
```

To restore:
```bash
docker run --rm -v portfolio_data:/data -v $(pwd):/backup alpine \
  tar xvf /backup/portfolio-backup.tar -C /
```

## 🔐 Security Recommendations

1. **Change Default Admin Password**
   - Login at `/admin` with `admin` / `admin123`
   - Change password immediately

2. **Firewall Configuration**
   ```bash
   sudo ufw allow 22    # SSH
   sudo ufw allow 80    # HTTP
   sudo ufw allow 443   # HTTPS
   sudo ufw enable
   ```

3. **Use HTTPS**
   - Follow the SSL setup guide above
   - Enable HSTS in Nginx

4. **Regular Updates**
   ```bash
   docker pull rust:latest
   ./deploy.sh prod
   ```

## ❗ Troubleshooting

### Container won't start
```bash
# Check logs
docker logs portfolio-app

# Check if port is in use
sudo lsof -i :8080
```

### Database issues
```bash
# Check database file permissions
docker exec portfolio-app ls -la /app/data/

# Reset database (WARNING: deletes all data)
docker volume rm portfolio_portfolio_data
./deploy.sh dev
```

### Nginx 502 Bad Gateway
```bash
# Check if portfolio app is running
docker ps
docker logs portfolio-app

# Check nginx logs
docker logs portfolio-nginx
```

## 📞 Support

For issues, please check the logs first:
```bash
./deploy.sh logs
```
