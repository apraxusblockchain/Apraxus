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

const POSITION_MANAGER =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

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

const deadline = Math.floor(Date.now() / 1000) + 3600;

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

console.log("=================================");
console.log("      V4 FINAL MINT CHECK");
console.log("=================================");
console.log("Wallet:", walletClient.account.address);
console.log("PositionManager:", POSITION_MANAGER);
console.log("---------------------------------");
console.log("Required WETH:", position.amount0.toExact());
console.log("Required APXS:", position.amount1.toExact());
console.log("Liquidity:", position.liquidity.toString());
console.log("Ticks:", "-46200 -> -45960");
console.log("Fee:", "3000");
console.log("Tick spacing:", "60");
console.log("Slippage:", "0.50%");
console.log("Deadline:", deadline);
console.log("---------------------------------");
console.log("Transaction value:", result.value);
console.log("Calldata length:", result.calldata.length);
console.log("Calldata prefix:", result.calldata.slice(0, 18));
console.log("---------------------------------");
console.log("Estimating gas only...");

const gas = await publicClient.estimateGas({
  account: walletClient.account.address,
  to: POSITION_MANAGER,
  data: result.calldata,
  value: BigInt(result.value),
});

console.log("Estimated gas:", gas.toString());
console.log("---------------------------------");
console.log("SENDING ACTUAL MINT TRANSACTION...");
console.log("---------------------------------");

const hash = await walletClient.sendTransaction({
  to: POSITION_MANAGER,
  data: result.calldata,
  value: BigInt(result.value),
  gas: gas,
});

console.log("Transaction hash:", hash);
console.log("Waiting for confirmation...");

const receipt = await publicClient.waitForTransactionReceipt({
  hash,
});

console.log("---------------------------------");
console.log("TRANSACTION CONFIRMED");
console.log("Status:", receipt.status);
console.log("Block:", receipt.blockNumber.toString());
console.log("Gas used:", receipt.gasUsed.toString());
console.log("=================================");
