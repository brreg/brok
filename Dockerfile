FROM node:18-slim AS base
WORKDIR /app
RUN npm install -g pnpm

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY packages/api/package.json packages/api/
COPY packages/captable/package.json packages/captable/
COPY packages/graph/package.json packages/graph/
RUN pnpm install --frozen-lockfile

COPY . .
RUN cp /app/packages/captable/.env.example /app/packages/captable/.env
RUN cp /app/packages/api/.env.example /app/packages/api/.env

RUN pnpm -F @brok/captable hardhat-compile-contracts-force
RUN pnpm -F @brok/graph prepare:localhost
RUN pnpm -F @brok/api build