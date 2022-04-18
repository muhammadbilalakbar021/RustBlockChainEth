import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert, expect } from "chai";
import { Tweet } from "../target/types/tweet";

describe("tweet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program: any = anchor.workspace.Tweet as Program<Tweet>;

  it("setup tweet platform!", async () => {
    const tweetKeypair = anchor.web3.Keypair.generate();
    const user: any = program.provider.wallet;
    await program.rpc.setupPlatform({
      accounts: {
        tweet: tweetKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [tweetKeypair],
    });

    let tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal("");
  });

  it("Write a tweet", async () => {
    const tweetKeypair = anchor.web3.Keypair.generate();
    const user = program.provider.wallet;
    await program.rpc.setupPlatform({
      accounts: {
        tweet: tweetKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [tweetKeypair],
    });

    let tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal("");

    await program.rpc.writeTweet("Hello World!", user.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);

    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal("Hello World!");
    expect(tweet.creator.toString()).to.equal(user.publicKey.toString());
  });

  it("should like tweet up no more than 5 times", async () => {
    const tweetKeypair = anchor.web3.Keypair.generate();
    const user = program.provider.wallet;
    await program.rpc.setupPlatform({
      accounts: {
        tweet: tweetKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [tweetKeypair],
    });

    let tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    // expect(tweet.message).to.equal("");

    await program.rpc.writeTweet("Hello World!", user.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal("Hello World!");
    expect(tweet.creator.toString()).to.equal(user.publicKey.toString());

    await program.rpc.likeTweet(user.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(1);
    expect(tweet.peopleWhoLiked[0].toString()).to.equal(
      user.publicKey.toString()
    );

    try {
      await program.rpc.likeTweet(user.publicKey, {
        accounts: {
          tweet: tweetKeypair.publicKey,
        },
        signers: [],
      });

      assert.ok(false);
    } catch (error) {
      console.log("error ", error.toString());
      const expectedError = "User has already liked the tweet";
      assert.equal(error.toString().toString(), expectedError);
    }

    const secondUser = anchor.web3.Keypair.generate();
    await program.rpc.likeTweet(secondUser.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(2);
    expect(tweet.peopleWhoLiked[1].toString()).to.equal(
      secondUser.publicKey.toString()
    );

    const thirdUser = anchor.web3.Keypair.generate();
    await program.rpc.likeTweet(thirdUser.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(3);
    expect(tweet.peopleWhoLiked[2].toString()).to.equal(
      thirdUser.publicKey.toString()
    );

    const fourthUser = anchor.web3.Keypair.generate();
    await program.rpc.likeTweet(fourthUser.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(4);
    expect(tweet.peopleWhoLiked[3].toString()).to.equal(
      fourthUser.publicKey.toString()
    );

    const fifthUser = anchor.web3.Keypair.generate();
    await program.rpc.likeTweet(fifthUser.publicKey, {
      accounts: {
        tweet: tweetKeypair.publicKey,
      },
      signers: [],
    });

    tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(5);
    expect(tweet.peopleWhoLiked[4].toString()).to.equal(
      fifthUser.publicKey.toString()
    );

    const sixthUser = anchor.web3.Keypair.generate();
    try {
      await program.rpc.likeTweet(sixthUser.publicKey, {
        accounts: {
          tweet: tweetKeypair.publicKey,
        },
        signers: [],
      });

      assert.ok(false);
    } catch (error) {
      console.log("error ", error.toString());
      assert.equal(
        error.toString().toString(),
        "Cannot receive more than 5 likes"
      );
    }
  });

  it("should not allow writting an empty message", async () => {
    const tweetKeypair = anchor.web3.Keypair.generate();
    const user = program.provider.wallet;
    await program.rpc.setupPlatform({
      accounts: {
        tweet: tweetKeypair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [tweetKeypair],
    });

    let tweet = await program.account.tweet.fetch(tweetKeypair.publicKey);
    expect(tweet.likes).to.equal(0);
    expect(tweet.message).to.equal("");

    try {
      await program.rpc.writeTweet("", user.publicKey, {
        accounts: {
          tweet: tweetKeypair.publicKey,
        },
        signers: [],
      });
      assert.ok(false);
    } catch (error) {
      assert.equal(error.toString().toString(), "Message cannot be empty");
    }
  });
});
