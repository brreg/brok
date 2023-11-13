#!/bin/bash
rm -rf \
node_modules/ \
packages/api/.next/ \
packages/api/node_modules/ \
packages/api/next-env.d.ts \
packages/captable/artifacts/ \
packages/captable/cache/ \
packages/captable/dist/ \
packages/captable/node_modules/ \
packages/captable/typechain-types/ \
packages/graph/node_modules/

pnpm install --frozen-lockfile
pnpm -F @brok/captable hardhat-compile-contracts
