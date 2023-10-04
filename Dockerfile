FROM node:18-slim AS base
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
RUN pnpm -F @brok/captable hardhat-compile-contracts-force
EXPOSE 8545

FROM hardhat AS graph
WORKDIR /app/packages/graph

FROM hardhat AS api
WORKDIR /app/packages/api
RUN cp .env.example .env
RUN pnpm build
EXPOSE 4000
CMD [ "pnpm", "start" ]

# FROM node:18-slim AS api
# WORKDIR /app
# COPY --from=api-builder /app/packages/api/package*.json ./
# COPY --from=api-builder /app/packages/api/.next ./.next
# COPY --from=api-builder /app/packages/api/public ./public
# COPY --from=api-builder /app/packages/api/node_modules ./node_modules
# RUN npm install -g pnpm
# ENV NODE_ENV=production