import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const {
  Token,
  CurrencyAmount,
} = require("@uniswap/sdk-core");

const {
  Pool,
  Position,
} = require("@uniswap/v4-sdk");

const chainId = 421614;

const WETH_ADDRESS =
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const APXS_ADDRESS =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5";

const WETH = new Token(
  chainId,
  WETH_ADDRESS,
  18,
  "WETH",
  "Wrapped Ether",
);

const APXS = new Token(
  chainId,
  APXS_ADDRESS,
  8,
  "APXS",
  "Apraxus",
);

console.log("=================================");
console.log("      APXS / WETH SDK TEST");
console.log("=================================");
console.log("Chain:", chainId);
console.log("WETH:", WETH.address);
console.log("WETH decimals:", WETH.decimals);
console.log("APXS:", APXS.address);
console.log("APXS decimals:", APXS.decimals);
console.log("---------------------------------");
console.log("V4 SDK loaded successfully.");
console.log("=================================");
