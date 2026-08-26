import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const JSBI = require("jsbi");
const { Token, CurrencyAmount } = require("@uniswap/sdk-core");
const { Pool, Position } = require("@uniswap/v4-sdk");
const { TickMath } = require("@uniswap/v3-sdk");

const chainId = 421614;

const WETH = new Token(
  chainId,
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73",
  18,
  "WETH"
);

const APXS = new Token(
  chainId,
  "0xFE16213961cb4f9B15301f730a5977b9A145add5",
  8,
  "APXS"
);

const sqrtCurrent = JSBI.BigInt(
  "7922816251426433759354395033"
);

const tickCurrent = -46055;
const tickLower = -46200;
const tickUpper = -45960;

const pool = new Pool(
  WETH,
  APXS,
  3000,
  60,
  "0x0000000000000000000000000000000000000000",
  sqrtCurrent,
  JSBI.BigInt("1374978937325881"),
  tickCurrent
);

const position = new Position({
  pool,
  liquidity: JSBI.BigInt("1374978937325881"),
  tickLower,
  tickUpper,
});

console.log("=================================");
console.log("      APXS / WETH POSITION");
console.log("=================================");
console.log("Current tick:", tickCurrent);
console.log("Lower tick:", tickLower);
console.log("Upper tick:", tickUpper);
console.log("---------------------------------");
console.log("Liquidity:", position.liquidity.toString());
console.log("Required WETH:", position.amount0.toExact());
console.log("Required APXS:", position.amount1.toExact());
console.log("---------------------------------");
console.log("Max test WETH: 0.0001");
console.log("Max test APXS: 10,000");
console.log("=================================");
