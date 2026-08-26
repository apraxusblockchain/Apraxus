import { network } from "hardhat";
import { formatUnits } from "viem";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();
const [walletClient] = await viem.getWalletClients();

const owner = walletClient.account.address;

const APXS = "0xFE16213961cb4f9B15301f730a5977b9A145add5";
const WETH = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73";

const PERMIT2 =
  "0x000000000022D473030F116dDEE9F6B43aC78BA3";

const POSITION_MANAGER =
  "0xAc631556d3d4019C95769033B5E719dD77124BAc";

const erc20Abi = [
  {
    type: "function",
    name: "allowance",
    stateMutability: "view",
    inputs: [
      { name: "owner", type: "address" },
      { name: "spender", type: "address" },
    ],
    outputs: [{ type: "uint256" }],
  },
  {
    type: "function",
    name: "balanceOf",
    stateMutability: "view",
    inputs: [{ name: "account", type: "address" }],
    outputs: [{ type: "uint256" }],
  },
] as const;

const permit2Abi = [
  {
    type: "function",
    name: "allowance",
    stateMutability: "view",
    inputs: [
      { name: "owner", type: "address" },
      { name: "token", type: "address" },
      { name: "spender", type: "address" },
    ],
    outputs: [
      { name: "amount", type: "uint160" },
      { name: "expiration", type: "uint48" },
      { name: "nonce", type: "uint48" },
    ],
  },
] as const;

async function checkToken(name: string, token: `0x${string}`, decimals: number) {
  const balance = await publicClient.readContract({
    address: token,
    abi: erc20Abi,
    functionName: "balanceOf",
    args: [owner],
  });

  const directAllowance = await publicClient.readContract({
    address: token,
    abi: erc20Abi,
    functionName: "allowance",
    args: [owner, PERMIT2],
  });

  const permitAllowance = await publicClient.readContract({
    address: PERMIT2,
    abi: permit2Abi,
    functionName: "allowance",
    args: [owner, token, POSITION_MANAGER],
  });

  console.log("---------------------------------");
  console.log(name);
  console.log("Balance:", formatUnits(balance, decimals));
  console.log(
    "ERC20 -> Permit2:",
    formatUnits(directAllowance, decimals),
  );
  console.log(
    "Permit2 -> PositionManager:",
    formatUnits(permitAllowance[0], decimals),
  );
  console.log("Permit2 expiration:", permitAllowance[1].toString());
  console.log("Permit2 nonce:", permitAllowance[2].toString());
}

console.log("=================================");
console.log("      V4 ALLOWANCE CHECK");
console.log("=================================");
console.log("Owner:", owner);
console.log("Permit2:", PERMIT2);
console.log("PositionManager:", POSITION_MANAGER);

await checkToken("WETH", WETH, 18);
await checkToken("APXS", APXS, 8);

console.log("=================================");
