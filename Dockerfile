FROM node:18 AS base
WORKDIR /app
RUN npm install -g pnpm

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY packages/api/package.json packages/api/
COPY packages/captable/package.json packages/captable/
COPY packages/graph/package.json packages/graph/
RUN pnpm install --frozen-lockfile
COPY . .

FROM base AS hardhat
WORKDIR /app/packages/captable
RUN cp .env.example .env
RUN pnpm hardhat-compile-contracts-force
EXPOSE 8545

FROM base AS graph
WORKDIR /app/packages/graph