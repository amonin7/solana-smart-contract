const anchor = require('@project-serum/anchor');

const main = async() => {
    console.log("🚀 Starting test...")

    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Contract;
    const tx = await program.rpc.startStuffOff();
    console.log("📝 Your transaction signature", tx);
    const tx2 = await program.rpc.anotherSimpleFunction();
    console.log("📝 Your transaction2 signature", tx2);
}

const runMain = async () => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
};

runMain();