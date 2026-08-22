import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { expect } from "chai";
import hre from "hardhat";
import { parseUnits } from "viem";

describe("APXS", function () {
  async function deployAPXS() {
    const { viem } = await hre.network.create();

    const publicClient = await viem.getPublicClient();
    const [holder, user] = await viem.getWalletClients();

    const apxs = await viem.deployContract("APXS", [
      holder.account.address,
    ]);

    return {
      apxs,
      publicClient,
      holder,
      user,
    };
  }

  it("should have the correct token metadata", async function () {
    const { apxs, publicClient } = await deployAPXS();

    expect(
      await publicClient.readContract({
        address: apxs.address,
        abi: apxs.abi,
        functionName: "name",
      }),
    ).to.equal("Apraxus");

    expect(
      await publicClient.readContract({
        address: apxs.address,
        abi: apxs.abi,
        functionName: "symbol",
      }),
    ).to.equal("APXS");

    expect(
      await publicClient.readContract({
        address: apxs.address,
        abi: apxs.abi,
        functionName: "decimals",
      }),
    ).to.equal(8);
  });

  it("should create exactly 1 billion APXS at deployment", async function () {
    const { apxs, publicClient, holder } = await deployAPXS();

    const maxSupply = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "MAX_SUPPLY",
    });

    const totalSupply = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "totalSupply",
    });

    const balance = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "balanceOf",
      args: [holder.account.address],
    });

    expect(maxSupply).to.equal(parseUnits("1000000000", 8));
    expect(totalSupply).to.equal(maxSupply);
    expect(balance).to.equal(maxSupply);
  });

  it("should reject a zero initial holder", async function () {
    const { viem } = await hre.network.create();

    await assert.rejects(
      viem.deployContract("APXS", [
        "0x0000000000000000000000000000000000000000",
      ]),
      /APXS: zero holder/,
    );
  });

  it("should allow normal ERC20 transfers", async function () {
    const { apxs, publicClient, holder, user } = await deployAPXS();

    const amount = parseUnits("100", 8);

    await apxs.write.transfer([user.account.address, amount], {
      account: holder.account,
    });

    const balance = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "balanceOf",
      args: [user.account.address],
    });

    expect(balance).to.equal(amount);
  });

  it("should allow holders to burn their own APXS", async function () {
    const { apxs, publicClient, holder, user } = await deployAPXS();

    const amount = parseUnits("100", 8);

    await apxs.write.transfer([user.account.address, amount], {
      account: holder.account,
    });

    await apxs.write.burn([parseUnits("40", 8)], {
      account: user.account,
    });

    const balance = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "balanceOf",
      args: [user.account.address],
    });

    const totalSupply = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "totalSupply",
    });

    expect(balance).to.equal(parseUnits("60", 8));
    expect(totalSupply).to.equal(
      parseUnits("999999960", 8),
    );
  });
});
