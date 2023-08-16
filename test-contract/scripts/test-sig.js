// interact.js

const API_KEY = process.env.API_KEY;
const PRIVATE_KEY = process.env.PRIVATE_KEY;
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDR;

const contract = require("../artifacts/contracts/Meh.sol/Meh.json");

// provider - Alchemy
const alchemyProvider = new ethers.providers.AlchemyProvider(network = "goerli", API_KEY);

// signer - you
const signer = new ethers.Wallet(PRIVATE_KEY, alchemyProvider);

// contract instance
const helloWorldContract = new ethers.Contract(CONTRACT_ADDRESS, contract.abi, signer);

async function main() {
    const message = ethers.utils.solidityKeccak256(["string"], ["hello"]);
    const sig = await signer.signMessage(ethers.utils.arrayify(message));

    console.log(sig);

    const signerAddr = await signer.getAddress();
    console.log('signer ADdr', signerAddr);
    const tx = await helloWorldContract.verify(signerAddr, "hello", sig)
    console.log(await tx.wait());
    // const message = await helloWorldContract.message();
    // console.log("The message is: " + message);

    // console.log("Updating the message...");
    // const tx = await helloWorldContract.update("this is the new message");
    // await tx.wait();

    // const newMessage = await helloWorldContract.message();
    // console.log("The new message is: " + newMessage);
}

main();
