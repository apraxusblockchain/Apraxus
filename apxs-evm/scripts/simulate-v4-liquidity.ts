import { createRequire } from "node:module";
import { network } from "hardhat";

const require = createRequire(import.meta.url);

const JSBI = require("jsbi");
const { Token, Percent } = require("@uniswap/sdk-core");
const { Pool, Position, V4PositionManager } =
  require("@uniswap/v4-sdk");

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

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

const position = new Position({
  pool: new Pool(
    WETH,
    APXS,
    3000,
    60,
    "0x0000000000000000000000000000000000000000",
    JSBI.BigInt("7922816251426433759354395033"),
    JSBI.BigInt("1374978937325881"),
    -46055,
  ),
  liquidity: JSBI.BigInt("1374978937325881"),
  tickLower: -46200,
  tickUpper: -45960,
});

const deadline =
  Math.floor(Date.now() / 1000) + 3600;

const result = V4PositionManager.addCallParameters(
  position,
  {
    recipient: walletClient.account.address,
    deadline,
    slippageTolerance: new Percent(50, 10_000),
    hookData: "0x",
    createPool: false,
    migrate: false,
  },
);

const positionManager =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

console.log("=================================");
console.log("    V4 LIQUIDITY SIMULATION");
console.log("=================================");
console.log("From:", walletClient.account.address);
console.log("PositionManager:", positionManager);
console.log("Required WETH:", position.amount0.toExact());
console.log("Required APXS:", position.amount1.toExact());
console.log("---------------------------------");
console.log("Simulating only...");
console.log("---------------------------------");

try {
  const resultCall = await publicClient.call({
    account: walletClient.account.address,
    to: positionManager,
    data: result.calldata,
    value: BigInt(result.value),
  });

  console.log("SIMULATION: SUCCESS");
  console.log("Return data:", resultCall.data);
} catch (error) {
  console.log("SIMULATION: REVERTED");
  console.log("---------------------------------");
  console.log(error);
}

console.log("=================================");
