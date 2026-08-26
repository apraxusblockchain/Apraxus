import { network } from "hardhat";

const { viem } = await network.create({
  network: "arbitrumSepolia",
  chainType: "l1",
});

const publicClient = await viem.getPublicClient();

const permit2 =
  "0x000000000022D473030F116dDEE9F6B43aC78BA3";

const bytecode = await publicClient.getCode({
  address: permit2,
});

console.log("=================================");
console.log("       PERMIT2 CONTRACT CHECK");
console.log("=================================");
console.log("Permit2:", permit2);
console.log("Code:", bytecode ? "FOUND" : "NOT FOUND");
console.log("Code length:", bytecode?.length ?? 0);
console.log("=================================");
