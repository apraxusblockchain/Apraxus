import { createRequire } from "node:module";

const require = createRequire(import.meta.url);

const JSBI = require("jsbi");
const { Token, Percent } = require("@uniswap/sdk-core");
const { Pool, Position, V4PositionManager } =
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

const pool = new Pool(
  WETH,
  APXS,
  3000,
  60,
  "0x0000000000000000000000000000000000000000",
  JSBI.BigInt("7922816251426433759354395033"),
  JSBI.BigInt("1374978937325881"),
  -46055,
);

const position = new Position({
  pool,
  liquidity: JSBI.BigInt("1374978937325881"),
  tickLower: -46200,
  tickUpper: -45960,
});

const deadline =
  Math.floor(Date.now() / 1000) + 3600;

const options = {
  recipient:
    "0x06433691c0AfD0341Df3F31a6C31637F7f86eE71",

  deadline,

  slippageTolerance: new Percent(50, 10_000),

  hookData: "0x",

  createPool: false,

  migrate: false,
};

const result =
  V4PositionManager.addCallParameters(
    position,
    options,
  );

console.log("=================================");
console.log("    V4 LIQUIDITY CALLDATA TEST");
console.log("=================================");
console.log("PositionManager:");
console.log(
  "0xAc631556d3d4019C95769033B5E719dD77124BAc",
);
console.log("---------------------------------");
console.log("Recipient:", options.recipient);
console.log("Deadline:", deadline);
console.log("Slippage: 0.50%");
console.log("---------------------------------");
console.log("Required WETH:", position.amount0.toExact());
console.log("Required APXS:", position.amount1.toExact());
console.log("---------------------------------");
console.log("Value:", result.value);
console.log("Calldata length:", result.calldata.length);
console.log("Calldata prefix:", result.calldata.slice(0, 18));
console.log("=================================");
