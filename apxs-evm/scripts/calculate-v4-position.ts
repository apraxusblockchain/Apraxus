import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const JSBI = require("jsbi");

const { Token, CurrencyAmount } =
  require("@uniswap/sdk-core");

const { Pool, Position } =
  require("@uniswap/v4-sdk");

const chainId = 421614;

const WETH = new Token(
  chainId,
  "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73",
  18,
  "WETH",
);

const APXS = new Token(
  chainId,
  "0xFE16213961cb4f9B15301f730a5977b9A145add5",
  8,
  "APXS",
);

const sqrtPriceX96 = JSBI.BigInt(
  "7922816251426433759354395033",
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
  sqrtPriceX96,
  JSBI.BigInt("0"),
  tickCurrent,
);

const amountWETH = CurrencyAmount.fromRawAmount(
  WETH,
  "100000000000000",
);

const amountAPXS = CurrencyAmount.fromRawAmount(
  APXS,
  "1000000000000",
);

const position = Position.fromAmounts({
  pool,
  tickLower,
  tickUpper,
  amount0: amountWETH,
  amount1: amountAPXS,
  useFullPrecision: true,
});

console.log("=================================");
console.log("     APXS / WETH LP CALCULATION");
console.log("=================================");
console.log("Current tick:", tickCurrent);
console.log("Lower tick:", tickLower);
console.log("Upper tick:", tickUpper);
console.log("---------------------------------");
console.log("Input WETH: 0.0001");
console.log("Input APXS: 10,000");
console.log("---------------------------------");
console.log("Liquidity:", position.liquidity.toString());
console.log("Required WETH:", position.amount0.toExact());
console.log("Required APXS:", position.amount1.toExact());
console.log("=================================");
