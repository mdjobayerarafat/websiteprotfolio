#!/bin/bash

# Portfolio Docker Deployment Script
# Usage: ./deploy.sh [dev|prod]

set -e

MODE=${1:-dev}

echo "🚀 Portfolio Deployment Script"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker first.${NC}"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo -e "${RED}❌ Docker Compose is not installed. Please install Docker Compose first.${NC}"
    exit 1
fi

# Use docker compose or docker-compose based on availability
if docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
else
    COMPOSE_CMD="docker-compose"
fi

case $MODE in
    dev)
        echo -e "${YELLOW}📦 Building and starting in DEVELOPMENT mode...${NC}"
        $COMPOSE_CMD down
        $COMPOSE_CMD build --no-cache
        $COMPOSE_CMD up -d
        echo -e "${GREEN}✅ Development server started!${NC}"
        echo -e "   Access at: http://localhost:8080"
        echo -e "   Admin panel: http://localhost:8080/admin"
        ;;
    prod)
        echo -e "${YELLOW}📦 Building and starting in PRODUCTION mode...${NC}"
        
        # Create required directories
        mkdir -p nginx/ssl certbot/conf certbot/www
        
        $COMPOSE_CMD -f docker-compose.prod.yml down
        $COMPOSE_CMD -f docker-compose.prod.yml build --no-cache
        $COMPOSE_CMD -f docker-compose.prod.yml up -d
        echo -e "${GREEN}✅ Production server started!${NC}"
        echo -e "   Access at: http://your-domain.com"
        echo -e "   Admin panel: http://your-domain.com/admin"
        ;;
    stop)
        echo -e "${YELLOW}🛑 Stopping all containers...${NC}"
        $COMPOSE_CMD down
        $COMPOSE_CMD -f docker-compose.prod.yml down 2>/dev/null || true
        echo -e "${GREEN}✅ All containers stopped!${NC}"
        ;;
    logs)
        echo -e "${YELLOW}📜 Showing logs...${NC}"
        $COMPOSE_CMD logs -f
        ;;
    *)
        echo "Usage: ./deploy.sh [dev|prod|stop|logs]"
        echo ""
        echo "Commands:"
        echo "  dev   - Start in development mode (default)"
        echo "  prod  - Start in production mode with Nginx"
        echo "  stop  - Stop all containers"
        echo "  logs  - Show container logs"
        exit 1
        ;;
esac

# Show running containers
echo ""
echo -e "${YELLOW}📊 Running containers:${NC}"
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
