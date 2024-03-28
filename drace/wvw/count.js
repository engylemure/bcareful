const readline = require('node:readline/promises');

const { stdin: input, stdout: output } = require('node:process');

const rl = readline.createInterface({ input, output });

/**
 * 
 * @param {number} time 
 * @returns 
 */
async function timeout(time) {
  return new Promise((res) => setTimeout(res, time))
}

let a = 0;

async function one() {
  await timeout(Math.random() * 10)
  return 1
}


async function add() {
  // a = await one() + a;
  a = a + await one();
}

async function main() {
  const countTo = Number(await rl.question("We should count until? "))
  const promises = []
  for (let i = 0; i < countTo; i++) {
    promises.push(add())
  }
  await Promise.all(promises)
  console.log(`we counted to ${a} but expects ${countTo}`)
}

main().then(() => process.exit(0))