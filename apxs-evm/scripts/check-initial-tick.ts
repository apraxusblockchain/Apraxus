const sqrtPriceX96 = 7922816251426433759354395033n;

const Q96 = 2n ** 96n;

const price = Number(sqrtPriceX96) / Number(Q96);
const rawPrice = price * price;

const tick = Math.log(rawPrice) / Math.log(1.0001);

console.log("=================================");
console.log("      APXS / WETH TICK CHECK");
console.log("=================================");
console.log("sqrtPriceX96:", sqrtPriceX96.toString());
console.log("Raw price (token1/token0):", rawPrice);
console.log("Calculated tick:", tick);
console.log("Nearest lower tick (spacing 60):", Math.floor(tick / 60) * 60);
console.log("Nearest upper tick (spacing 60):", Math.ceil(tick / 60) * 60);
console.log("=================================");
