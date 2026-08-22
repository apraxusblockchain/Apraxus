import { network } from "hardhat";

const { viem } = await network.create({
  network: "sepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const address =
  "0xFE16213961cb4f9B15301f730a5977b9A145add5" as `0x${string}`;

const abi = [
  {
    type: "function",
    name: "name",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "string" }],
  },
  {
    type: "function",
    name: "symbol",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "string" }],
  },
  {
    type: "function",
    name: "decimals",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "uint8" }],
  },
  {
    type: "function",
    name: "totalSupply",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "uint256" }],
  },
  {
    type: "function",
    name: "MAX_SUPPLY",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "uint256" }],
  },
  {
    type: "function",
    name: "owner",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "address" }],
  },
] as const;

const name = await publicClient.readContract({
  address,
  abi,
  functionName: "name",
});

const symbol = await publicClient.readContract({
  address,
  abi,
  functionName: "symbol",
});

const decimals = await publicClient.readContract({
  address,
  abi,
  functionName: "decimals",
});

const totalSupply = await publicClient.readContract({
  address,
  abi,
  functionName: "totalSupply",
});

const maxSupply = await publicClient.readContract({
  address,
  abi,
  functionName: "MAX_SUPPLY",
});

const owner = await publicClient.readContract({
  address,
  abi,
  functionName: "owner",
});

console.log("=================================");
console.log("       APXS SEPOLIA CHECK");
console.log("=================================");
console.log("Contract:", address);
console.log("Name:", name);
console.log("Symbol:", symbol);
console.log("Decimals:", decimals);
console.log("Total Supply:", totalSupply.toString());
console.log("Max Supply:", maxSupply.toString());
console.log("Owner:", owner);
console.log("=================================");
