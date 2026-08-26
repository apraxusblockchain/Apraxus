import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const POSITION_MANAGER =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const TOKEN_ID = 502n;

const abi = [
  {
    type: "function",
    name: "getPositionLiquidity",
    stateMutability: "view",
    inputs: [{ name: "tokenId", type: "uint256" }],
    outputs: [{ name: "liquidity", type: "uint128" }],
  },
  {
    type: "function",
    name: "getPoolAndPositionInfo",
    stateMutability: "view",
    inputs: [{ name: "tokenId", type: "uint256" }],
    outputs: [
      {
        name: "poolKey",
        type: "tuple",
        components: [
          { name: "currency0", type: "address" },
          { name: "currency1", type: "address" },
          { name: "fee", type: "uint24" },
          { name: "tickSpacing", type: "int24" },
          { name: "hooks", type: "address" },
        ],
      },
      { name: "info", type: "uint256" },
    ],
  },
  {
    type: "function",
    name: "ownerOf",
    stateMutability: "view",
    inputs: [{ name: "tokenId", type: "uint256" }],
    outputs: [{ name: "", type: "address" }],
  },
] as const;

console.log("=================================");
console.log("   APXS/WETH POSITION #502");
console.log("=================================");
console.log("PositionManager:", POSITION_MANAGER);
console.log("Token ID:", TOKEN_ID.toString());
console.log("---------------------------------");

const owner = await publicClient.readContract({
  address: POSITION_MANAGER,
  abi,
  functionName: "ownerOf",
  args: [TOKEN_ID],
});

const liquidity = await publicClient.readContract({
  address: POSITION_MANAGER,
  abi,
  functionName: "getPositionLiquidity",
  args: [TOKEN_ID],
});

const [poolKey, info] = await publicClient.readContract({
  address: POSITION_MANAGER,
  abi,
  functionName: "getPoolAndPositionInfo",
  args: [TOKEN_ID],
});

console.log("Owner:", owner);
console.log("Liquidity:", liquidity.toString());
console.log("---------------------------------");
console.log("Currency0:", poolKey.currency0);
console.log("Currency1:", poolKey.currency1);
console.log("Fee:", poolKey.fee);
console.log("Tick spacing:", poolKey.tickSpacing);
console.log("Hooks:", poolKey.hooks);
console.log("---------------------------------");
console.log("Packed PositionInfo:", info.toString());
console.log("Packed PositionInfo HEX:", "0x" + info.toString(16));
console.log("=================================");
