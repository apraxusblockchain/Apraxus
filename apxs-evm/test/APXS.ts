import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { expect } from "chai";
import hre from "hardhat";
import { parseUnits } from "viem";

describe("APXS", function () {
  async function deployAPXS() {
    const { viem } = await hre.network.connect();

    const publicClient = await viem.getPublicClient();
    const [owner, user, other] = await viem.getWalletClients();

    const apxs = await viem.deployContract("APXS", [owner.account.address]);

    return {
      apxs,
      publicClient,
      owner,
      user,
      other,
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

  it("should have a maximum supply of 1 billion APXS", async function () {
    const { apxs, publicClient } = await deployAPXS();

    const maxSupply = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "MAX_SUPPLY",
    });

    expect(maxSupply).to.equal(parseUnits("1000000000", 8));
  });

  it("should set the deployer as owner", async function () {
    const { apxs, publicClient, owner } = await deployAPXS();

    const contractOwner = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "owner",
    });

    expect(contractOwner.toLowerCase()).to.equal(
      owner.account.address.toLowerCase(),
    );
  });

  it("should allow the owner to mint within the maximum supply", async function () {
    const { apxs, publicClient, owner, user } = await deployAPXS();

    const amount = parseUnits("100", 8);

    await apxs.write.mint([user.account.address, amount], {
      account: owner.account,
    });

    const balance = await publicClient.readContract({
      address: apxs.address,
      abi: apxs.abi,
      functionName: "balanceOf",
      args: [user.account.address],
    });

    expect(balance).to.equal(amount);
  });

  it("should reject minting above the maximum supply", async function () {
    const { apxs, owner, user } = await deployAPXS();

    const maxSupply = parseUnits("1000000000", 8);

    await assert.rejects(
      apxs.write.mint([user.account.address, maxSupply + 1n], {
        account: owner.account,
      }),
    );
  });

  it("should reject minting by a non-owner", async function () {
    const { apxs, user, other } = await deployAPXS();

    const amount = parseUnits("100", 8);

    await assert.rejects(
      apxs.write.mint([other.account.address, amount], {
        account: user.account,
      }),
    );
  });

  it("should allow holders to burn their own APXS", async function () {
    const { apxs, publicClient, owner, user } = await deployAPXS();

    const amount = parseUnits("100", 8);

    await apxs.write.mint([user.account.address, amount], {
      account: owner.account,
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

    expect(balance).to.equal(parseUnits("60", 8));
  });
});